/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import {
    IVehicleState,
    Uuid,
} from "@/models";

class Settings {
    clear() {
        localStorage.clear();
    }

    get vehicleId(): Uuid | null {
        return localStorage.getItem("vehicleId") as Uuid | null;
    }

    set vehicleId(value: Uuid | null) {
        if(value) {
            localStorage.setItem("vehicleId", value);
        } else {
            localStorage.removeItem("vehicleId");
        }
    }

    get vehicleState(): IVehicleState | null {
        const value = localStorage.getItem("vehicleState");
        return value ? JSON.parse(value) : null;
    }

    set vehicleState(value: IVehicleState | null) {
        if(value) {
            localStorage.setItem("vehicleState", JSON.stringify(value));
        } else {
            localStorage.removeItem("vehicleState");
        }
    }

    get flipCameraStream(): boolean {
        return localStorage.getItem("flipCameraStream") === "true";
    }

    set flipCameraStream(value: boolean) {
        localStorage.setItem("flipCameraStream", value.toString());
    }
}

export default new Settings();
