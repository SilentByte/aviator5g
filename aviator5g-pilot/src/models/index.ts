/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import { DateTime } from "luxon";

export type Opaque<K, T> = T & { __TYPE__: K };
export type Uuid = Opaque<"Uuid", string>;

export interface IVehicleState {
    ailerons: number;
    elevator: number;
    rudder: number;
    throttle: number;
}

export interface IVehicleConfig {
    aileronsTrim: number;
    aileronsReverse: boolean;
    elevatorTrim: number;
    elevatorReverse: boolean;
    rudderTrim: number;
    rudderReverse: boolean;
    throttleTrim: number;
    throttleReverse: boolean;
}
