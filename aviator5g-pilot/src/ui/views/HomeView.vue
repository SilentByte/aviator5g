<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <div>
        <div class="state-container">
            <table>
                <tr>
                    <td>ID:</td>
                    <td><small>{{ app.vehicleId }}</small></td>
                </tr>
                <tr>
                    <td>A:</td>
                    <td>{{
                            formatValue(
                                app.vehicleState.aileronsValue,
                                app.vehicleState.aileronsTrim,
                                app.vehicleState.aileronsReverse,
                            )
                        }}
                    </td>
                </tr>
                <tr>
                    <td>E:</td>
                    <td>{{
                            formatValue(
                                app.vehicleState.elevatorValue,
                                app.vehicleState.elevatorTrim,
                                app.vehicleState.elevatorReverse,
                            )
                        }}
                    </td>
                </tr>
                <tr>
                    <td>R:</td>
                    <td>{{
                            formatValue(
                                app.vehicleState.rudderValue,
                                app.vehicleState.rudderTrim,
                                app.vehicleState.rudderReverse,
                            )
                        }}
                    </td>
                </tr>
                <tr>
                    <td>T:</td>
                    <td>{{
                            formatValue(
                                app.vehicleState.throttleValue,
                                app.vehicleState.throttleTrim,
                                app.vehicleState.throttleReverse,
                            )
                        }}
                    </td>
                </tr>
            </table>
        </div>

        <VirtualJoystick class="left-stick"
                         :rest-y="false"
                         :size="200"
                         @move="onMoveLeftStick" />

        <VirtualJoystick class="right-stick"
                         :size="200"
                         @move="onMoveRightStick" />
    </div>
</template>

<!--suppress JSMethodCanBeStatic -->
<script lang="ts">

import {
    Component,
    Vue,
} from "vue-property-decorator";

import { getModule } from "vuex-module-decorators";
import { AppModule } from "@/store/app";

import VirtualJoystick, { IVirtualJoystickEvent } from "@/ui/components/VirtualJoystick.vue";

function formatNumberWithSign(value: number, decimals: number): string {
    const v = Math.abs(value);
    const s = value === 0 ? "=" : value > 0 ? "+" : "-";

    return `${s}${v.toFixed(decimals)}`;
}

@Component({
    components: {
        VirtualJoystick,
    },
})
export default class HomeView extends Vue {
    private readonly app = getModule(AppModule);

    private formatValue(value: number, trim: number, reverse: boolean): string {
        const sv = formatNumberWithSign(value, 6);
        const st = formatNumberWithSign(trim, 6);
        const sr = reverse ? "REV" : "";

        return `${sv} (${st}) ${sr}`;
    }

    private onMoveLeftStick(e: IVirtualJoystickEvent) {
        this.app.doUpdateVehicleState({
            state: {
                rudderValue: e.vector.x,
                throttleValue: e.vector.y,
            },
        });
    }

    private onMoveRightStick(e: IVirtualJoystickEvent) {
        this.app.doUpdateVehicleState({
            state: {
                aileronsValue: e.vector.x,
                elevatorValue: e.vector.y,
            },
        });
    }
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

.state-container {
    display: inline-block;
    margin: 12px;
    padding: 12px;

    color: white;
    background-color: rgba(0, 0, 0, 0.4);
    border-radius: 4px;

    font-size: 14px;
    font-family: Consolas, monospace;

    table {
        tr td:nth-child(1) {
            text-align: right;
        }
    }
}

.left-stick {
    position: absolute;
    left: 12px;
    bottom: 12px;
    width: 200px;
    height: 200px;
}

.right-stick {
    position: absolute;
    right: 12px;
    bottom: 12px;
    width: 200px;
    height: 200px;
}

</style>
