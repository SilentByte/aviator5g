/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import settings from "@/store/settings";

export type Opaque<K, T> = T & { __TYPE__: K };
export type Uuid = Opaque<"Uuid", string>;

export interface IVehicleState {
    aileronsValue: number;
    aileronsTrim: number;
    aileronsReverse: boolean;

    elevatorValue: number;
    elevatorTrim: number;
    elevatorReverse: boolean;

    rudderValue: number;
    rudderTrim: number;
    rudderReverse: boolean;

    throttleValue: number;
    throttleTrim: number;
    throttleReverse: boolean;
}

export function defaultVehicleState(): IVehicleState {
    return {
        aileronsTrim: 0.0,
        aileronsReverse: false,

        elevatorTrim: 0.0,
        elevatorReverse: false,

        rudderTrim: 0.0,
        rudderReverse: false,

        throttleTrim: 0.0,
        throttleReverse: false,

        ...settings.vehicleState,

        aileronsValue: 0.0,
        elevatorValue: 0.0,
        rudderValue: 0.0,
        throttleValue: 0.0,
    };
}
