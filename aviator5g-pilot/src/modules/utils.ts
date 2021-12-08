/*
 * AVIATOR 5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

import { v4 as uuid4gen } from "uuid";
import { Uuid } from "@/models";

export function uuid4(): Uuid {
    return uuid4gen() as Uuid;
}
