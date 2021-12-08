/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import ReconnectingWebSocket from "reconnecting-websocket";

import {
    config as VuexModuleDecoratorsConfig,
    Action,
    Module,
    Mutation,
    VuexModule,
} from "vuex-module-decorators";

import store from "@/store";

import * as utils from "@/modules/utils";
import settings from "@/store/settings";

import {
    defaultVehicleState,
    IVehicleState, Uuid,
} from "@/models";

VuexModuleDecoratorsConfig.rawError = true;

const DEFAULT_GROUP_ID: Uuid = "14ed4af8-5256-4e74-a5d6-545dfc0b004c" as Uuid;

function calculateAxisValue(value: number, trim: number, reverse: boolean): number {
    const r = reverse ? +1 : -1;
    return value * r + trim;
}

@Module({
    store,
    dynamic: true,
    namespaced: true,
    name: "app",
})
export class AppModule extends VuexModule {
    private rws: ReconnectingWebSocket | null = null;

    vehicleId = utils.uuid4();
    vehicleState: IVehicleState = defaultVehicleState();

    @Mutation
    initializeStore(): void {
        this.vehicleId = settings.vehicleId || utils.uuid4();
        settings.vehicleId = this.vehicleId;

        if(this.rws) {
            this.rws.close();
        }

        this.rws = new ReconnectingWebSocket("ws://192.168.0.80:9000");
        this.rws.addEventListener("open", () => {
            this.rws?.send(JSON.stringify({
                "type": "identification",
                "group_id": DEFAULT_GROUP_ID,
                "id": this.vehicleId,
                "client_type": "pilot",
            }));
        });
    }

    @Mutation
    uninitializeStore(): void {
        if(this.rws) {
            this.rws.close();
        }
    }

    @Mutation
    updateVehicleState(state: Partial<IVehicleState>): void {
        if(state.aileronsValue !== undefined) {
            this.vehicleState.aileronsValue = state.aileronsValue;
        }

        if(state.elevatorValue !== undefined) {
            this.vehicleState.elevatorValue = state.elevatorValue;
        }

        if(state.rudderValue !== undefined) {
            this.vehicleState.rudderValue = state.rudderValue;
        }

        if(state.throttleValue !== undefined) {
            this.vehicleState.throttleValue = state.throttleValue;
        }
    }

    @Action
    doUpdateVehicleState(payload: { state: Partial<IVehicleState> }): void {
        this.context.commit("updateVehicleState", payload.state);
        this.rws?.send(JSON.stringify({
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
