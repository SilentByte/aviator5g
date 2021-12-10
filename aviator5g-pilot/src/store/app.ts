/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import { DateTime, Duration } from "luxon";
import ReconnectingWebSocket from "reconnecting-websocket";

import {
    config as VuexModuleDecoratorsConfig,
    Action,
    Module,
    Mutation,
    VuexModule,
    getModule,
} from "vuex-module-decorators";

import store from "@/store";

import * as utils from "@/modules/utils";
import settings from "@/store/settings";

import {
    defaultVehicleState,
    IVehicleState,
    Uuid,
} from "@/models";

VuexModuleDecoratorsConfig.rawError = true;

const SOCKET_ENDPOINT = process.env.VUE_APP_SOCKET_ENDPOINT_URL;
const CAMERA_STREAM_ENDPOINT = process.env.VUE_APP_CAMERA_STREAM_ENDPOINT_URL;

const DEFAULT_GROUP_ID: Uuid = process.env.VUE_APP_DEFAULT_GROUP_ID;
const LATENCY_CHECK_INTERVAL_MS = 2000;

function calculateAxisValue(value: number, trim: number, reverse: boolean): number {
    const r = reverse ? +1 : -1;
    return value * r + trim * r;
}

@Module({
    store,
    dynamic: true,
    namespaced: true,
    name: "app",
})
export class AppModule extends VuexModule {
    private rws: ReconnectingWebSocket | null = null;
    private latencyInterval = 0;

    cameraStreamEndpointUrl = CAMERA_STREAM_ENDPOINT;

    isConnected = false;
    roundTripLatency: Duration = Duration.fromMillis(0);

    vehicleId = utils.uuid4();
    vehicleState: IVehicleState = defaultVehicleState();

    @Mutation
    initializeStore(): void {
        this.vehicleId = settings.vehicleId || utils.uuid4();
        settings.vehicleId = this.vehicleId;

        if(this.rws) {
            this.rws.close();
        }

        this.rws = new ReconnectingWebSocket(SOCKET_ENDPOINT);

        const app = getModule(AppModule);
        this.rws.addEventListener("open", () => app.doHandleOpenConnection());
        this.rws.addEventListener("close", () => app.doHandleCloseConnection());
        this.rws.addEventListener("message", e => app.doHandleMessage(e));

        this.latencyInterval = setInterval(() => app.doSendLatencyRequest(), LATENCY_CHECK_INTERVAL_MS);
    }

    @Mutation
    uninitializeStore(): void {
        if(this.latencyInterval) {
            clearInterval(this.latencyInterval);
            this.latencyInterval = 0;
        }

        if(this.rws) {
            this.rws.close();
        }
    }

    @Mutation
    setConnectionState(isConnected: boolean): void {
        this.isConnected = isConnected;
    }

    @Mutation
    setRoundTripLatency(roundTripLatency: Duration): void {
        this.roundTripLatency = roundTripLatency;
    }

    @Mutation
    updateVehicleState(state: Partial<IVehicleState>): void {
        Object.assign(this.vehicleState, state);
        settings.vehicleState = this.vehicleState;
    }

    @Action
    doHandleOpenConnection(): void {
        this.context.commit("setConnectionState", true);
        this.doSendIdentification();
    }

    @Action
    doHandleCloseConnection(): void {
        this.context.commit("setConnectionState", false);
    }

    @Action
    doHandleMessage(e: MessageEvent): void {
        const message = JSON.parse(e.data);

        if(message.type === "latency_response") {
            const timestamp = DateTime.fromISO(message.timestamp);
            this.context.commit("setRoundTripLatency", DateTime.now().diff(timestamp));
        }
    }

    @Action
    doSendIdentification(): void {
        if(!this.isConnected || !this.rws) {
            return;
        }

        this.rws.send(JSON.stringify({
            "type": "identification",
            "group_id": DEFAULT_GROUP_ID,
            "id": this.vehicleId,
            "client_type": "pilot",
        }));
    }

    @Action
    doSendLatencyRequest(): void {
        if(!this.isConnected || !this.rws) {
            return;
        }

        this.rws.send(JSON.stringify({
            "type": "latency_request",
            "initiator_id": this.vehicleId,
            "timestamp": DateTime.now().toISO(),
        }));
    }

    @Action
    doUpdateVehicleState(state: Partial<IVehicleState>): void {
        this.context.commit("updateVehicleState", state);

        if(!this.isConnected || !this.rws) {
            return;
        }

        this.rws.send(JSON.stringify({
            type: "control",
            axes: [
                calculateAxisValue(
                    this.vehicleState.aileronsValue,
                    this.vehicleState.aileronsTrim,
                    this.vehicleState.aileronsReverse,
                ),
                calculateAxisValue(
                    this.vehicleState.elevatorValue,
                    this.vehicleState.elevatorTrim,
                    this.vehicleState.elevatorReverse,
                ),
                calculateAxisValue(
                    this.vehicleState.rudderValue,
                    this.vehicleState.rudderTrim,
                    this.vehicleState.rudderReverse,
                ),
                calculateAxisValue(
                    this.vehicleState.throttleValue,
                    this.vehicleState.throttleTrim,
                    this.vehicleState.throttleReverse,
                ),
            ],
        }));
    }
}
