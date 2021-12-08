/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

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
        aileronsValue: 0.0,
        aileronsTrim: 0.0,
        aileronsReverse: false,

        elevatorValue: 0.0,
        elevatorTrim: 0.0,
        elevatorReverse: false,

        rudderValue: 0.0,
        rudderTrim: 0.0,
        rudderReverse: false,

        throttleValue: 0.0,
        throttleTrim: 0.0,
        throttleReverse: false,
    };
}
