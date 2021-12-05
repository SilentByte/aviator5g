/*
 * AVIATOR5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        Arc,
        Mutex,
    },
};

use argh::FromArgs;
use aviator5g_common::ControlMessage;
use futures_channel::mpsc::{
    unbounded,
    UnboundedSender,
};
use futures_util::{
    future,
    pin_mut,
    stream::TryStreamExt,
    StreamExt,
};
use tokio::net::{
    TcpListener,
    TcpStream,
};

type Tx = UnboundedSender<tungstenite::Message>;

struct ConnectionState {
    tx: Tx,
    group_id: Option<aviator5g_common::Id>,
    id: Option<aviator5g_common::Id>,
    client_type: Option<aviator5g_common::ClientType>,
}

impl ConnectionState {
    fn new(tx: Tx) -> Self {
        Self {
            tx,
            group_id: None,
            id: None,
            client_type: None,
        }
    }

    fn is_identified(&self) -> bool {
        self.group_id.is_some() && self.id.is_some() && self.client_type.is_some()
    }

    fn identify(
        &mut self,
        group_id: aviator5g_common::Id,
        id: aviator5g_common::Id,
        client_type: aviator5g_common::ClientType,
    ) {
        self.group_id = Some(group_id);
        self.id = Some(id);
        self.client_type = Some(client_type);
    }
}

struct ServerState {
    connections: HashMap<SocketAddr, ConnectionState>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn accept_connection(&mut self, address: SocketAddr, tx: Tx) {
        self.connections.insert(address, ConnectionState::new(tx));
    }

    fn release_connection(&mut self, address: &SocketAddr) {
        self.connections.remove(address);
    }

    fn connection_from_socket_address(&self, address: SocketAddr) -> Option<&ConnectionState> {
        self.connections.get(&address)
    }

    fn connection_from_socket_address_mut(
        &mut self,
        address: SocketAddr,
    ) -> Option<&mut ConnectionState> {
        self.connections.get_mut(&address)
    }

    fn connection_from_id(&self, id: aviator5g_common::Id) -> Option<&ConnectionState> {
        self.connections.values().into_iter().find(|v| v.id == Some(id))
    }
}

#[derive(thiserror::Error)]
pub enum ServerError {
    #[error("An error occurred while handling the connection: {0}")]
    ConnectionError(tungstenite::Error),

    #[error("The control message is malformed: {0}")]
    MalformedControlMessageError(String),

    #[error("Client is not identified")]
    NotIdentifiedError,

    #[error("Client has already been identified")]
    AlreadyIdentifiedError,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<tungstenite::Error> for ServerError {
    fn from(e: tungstenite::Error) -> Self {
        Self::ConnectionError(e)
    }
}

impl std::fmt::Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        aviator5g_common::error_chain_fmt(self, f)
    }
}

/// Aviator5G Server.
#[derive(Debug, Clone, FromArgs)]
struct Args {
    /// the hostname on which the server listens.
    #[argh(option, default = r#""localhost".into()"#)]
    host: String,

    /// the server's port.
    #[argh(option, default = "9000")]
    port: u16,
}

pub enum ControlMessageAction {
    None,
    ForwardAll,
    ForwardSingle(aviator5g_common::Id),
}

fn handle_control_message(
    server_state: Arc<Mutex<ServerState>>,
    socket_address: SocketAddr,
    control_message: ControlMessage,
) -> Result<ControlMessageAction, ServerError> {
    log::debug!(
        "Handling control message: {} {:?}",
        socket_address,
        control_message
    );

    let mut server_state = server_state.lock().unwrap();
    let connection = server_state
        .connection_from_socket_address_mut(socket_address)
        .expect("Unknown connection");

    match control_message {
        ControlMessage::Identification(e) => {
            if connection.is_identified() {
                return Err(ServerError::AlreadyIdentifiedError);
            }

            connection.identify(e.group_id, e.id, e.client_type);
            Ok(ControlMessageAction::None)
        }

        ControlMessage::Control(_) | ControlMessage::LatencyRequest(_) => {
            if !connection.is_identified() {
                return Err(ServerError::NotIdentifiedError);
            }
            Ok(ControlMessageAction::ForwardAll)
        }

        ControlMessage::LatencyResponse(e) => {
            if !connection.is_identified() {
                return Err(ServerError::NotIdentifiedError);
            }

            Ok(ControlMessageAction::ForwardSingle(e.initiator_id))
        }
    }
}

fn handle_message(
    server_state: Arc<Mutex<ServerState>>,
    socket_address: SocketAddr,
    message: tungstenite::Message,
) -> Result<ControlMessageAction, ServerError> {
    match &message {
        tungstenite::Message::Text(text) => {
            log::debug!("Received WS Text: {}", socket_address);

            let control_message = aviator5g_common::parse_control_message(&text)
                .map_err(|e| ServerError::MalformedControlMessageError(e))?;

            handle_control_message(server_state, socket_address, control_message)
        }
        tungstenite::Message::Binary(_) => {
            log::debug!("Received Binary Message: {}", socket_address);
            Ok(ControlMessageAction::None)
        }
        tungstenite::Message::Ping(_) => {
            log::debug!("Received Ping Message: {}", socket_address);
            Ok(ControlMessageAction::None)
        }
        tungstenite::Message::Pong(_) => {
            log::debug!("Received Pong Message: {}", socket_address);
            Ok(ControlMessageAction::None)
        }
        tungstenite::Message::Close(_) => {
            log::debug!("Received Close Message: {}", socket_address);
            Ok(ControlMessageAction::None)
        }
    }
}

async fn handle_connection(
    server_state: Arc<Mutex<ServerState>>,
    tcp_stream: TcpStream,
    socket_address: SocketAddr,
) -> anyhow::Result<(), ServerError> {
    log::info!("Incoming TCP connection: {}", socket_address);

    let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await?;

    log::info!("WebSocket connection established: {}", socket_address);

    let (tx, rx) = unbounded();
    server_state
        .lock()
        .unwrap()
        .accept_connection(socket_address, tx);

    let (outgoing, incoming) = ws_stream.split();

    let (abort_connection_tx, abort_connection_rx) = futures::channel::oneshot::channel::<()>();
    let abort_connection = Mutex::new(Some(abort_connection_tx));

    let broadcast_incoming = incoming.try_for_each(|message| {
        match handle_message(server_state.clone(), socket_address, message.clone()) {
            Ok(action) => match action {
                ControlMessageAction::None => {}
                ControlMessageAction::ForwardAll => {
                    let server_state = server_state.lock().unwrap();
                    let current_connection = &server_state
                        .connection_from_socket_address(socket_address)
                        .expect("Unknown connection");

                    let current_group_id = &current_connection.group_id;
                    let client_type = &current_connection.client_type;

                    // Forward message to all other clients of different type within the same group.
                    server_state
                        .connections
                        .iter()
                        .filter(|(sa, state)| {
                            sa != &&socket_address
                                && state.group_id == *current_group_id
                                && state.client_type != *client_type
                        })
                        .map(|(_, state)| &state.tx)
                        .for_each(|tx| tx.unbounded_send(message.clone()).unwrap());
                }
                ControlMessageAction::ForwardSingle(recipient_id) => {
                    let server_state = server_state.lock().unwrap();
                    if let Some(connection) = server_state.connection_from_id(recipient_id) {
                        connection.tx.unbounded_send(message.clone()).unwrap();
                    }
                }
            },
            Err(e) => {
                log::error!(
                    "An error occurred while handling the control message: {} {:?} ::: {}",
                    socket_address,
                    e,
                    message,
                );

                abort_connection
                    .lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .send(())
                    .unwrap();
            }
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(
        future::select(broadcast_incoming, receive_from_others),
        abort_connection_rx,
    )
    .await;

    log::info!("Connection disconnected: {}", &socket_address);
    server_state
        .lock()
        .unwrap()
        .release_connection(&socket_address);

    Ok(())
}

async fn handle_connection_wrapper(
    server_state: Arc<Mutex<ServerState>>,
    tcp_stream: TcpStream,
    socket_address: SocketAddr,
) {
    let result = handle_connection(server_state.clone(), tcp_stream, socket_address).await;
    if let Err(e) = result {
        log::error!(
            "Connection {} has been terminated due to an error: {:?}",
            socket_address,
            e
        );
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Args = argh::from_env();
    let server_state = Arc::new(Mutex::new(ServerState::new()));

    log::info!("Starting server at {}:{}...", args.host, args.port);
    let server = TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;

    while let Ok((tcp_stream, socket_address)) = server.accept().await {
        tokio::spawn(handle_connection_wrapper(
            server_state.clone(),
            tcp_stream,
            socket_address,
        ));
    }

    Ok(())
}
