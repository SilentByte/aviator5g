/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import { Uuid } from "@/models";

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
}

export default new Settings();
