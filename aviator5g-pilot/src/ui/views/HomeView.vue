<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <div>
        <div class="state-container">
            <table>
                <tr>
                    <td>V:</td>
                    <td>{{ app.vehicleId }}</td>
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

        <v-slider dense hide-details
                  class="left-stick-trim-horizontal"
                  color="accent"
                  track-color="accent"
                  min="-1"
                  max="+1"
                  step="0.001"
                  :value="app.vehicleState.rudderTrim"
                  @change="onLeftStickTrimHorizontalChange" />

        <v-slider dense vertical hide-details
                  class="left-stick-trim-vertical"
                  color="accent"
                  track-color="accent"
                  min="-1"
                  max="+1"
                  step="0.001"
                  :value="app.vehicleState.throttleTrim"
                  @change="onLeftStickTrimVerticalChange" />

        <v-slider dense hide-details
                  class="right-stick-trim-horizontal"
                  color="accent"
                  track-color="accent"
                  min="-1"
                  max="+1"
                  step="0.001"
                  :value="app.vehicleState.aileronsTrim"
                  @change="onRightStickTrimHorizontalChange" />

        <v-slider dense vertical hide-details
                  class="right-stick-trim-vertical"
                  color="accent"
                  track-color="accent"
                  min="-1"
                  max="+1"
                  step="0.001"
                  :value="app.vehicleState.throttleTrim"
                  @change="onRightStickTrimVerticalChange" />

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
            rudderValue: e.vector.x,
            throttleValue: e.vector.y,
        });
    }

    private onMoveRightStick(e: IVirtualJoystickEvent) {
        this.app.doUpdateVehicleState({
            aileronsValue: e.vector.x,
            elevatorValue: e.vector.y,
        });
    }

    private onLeftStickTrimHorizontalChange(value: number) {
        this.app.doUpdateVehicleState({
            rudderTrim: value,
        });
    }

    private onLeftStickTrimVerticalChange(value: number) {
        this.app.doUpdateVehicleState({
            throttleTrim: value,
        });
    }

    private onRightStickTrimHorizontalChange(value: number) {
        this.app.doUpdateVehicleState({
            aileronsTrim: value,
        });
    }

    private onRightStickTrimVerticalChange(value: number) {
        this.app.doUpdateVehicleState({
            elevatorTrim: value,
        });
    }
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

$margin-width: 12px;
$stick-size: 200px;

.state-container {
    display: inline-block;
    margin: $margin-width;
    padding: $margin-width / 2;

    color: white;
    background-color: rgba(0, 0, 0, 0.3);
    border-radius: 4px;

    font-size: 8px;
    font-family: Consolas, monospace;
    line-height: 8px;

    table {
        tr td:nth-child(1) {
            text-align: right;
        }
    }
}

.left-stick {
    z-index: 0;
    position: absolute;
    left: $margin-width;
    bottom: $margin-width;
    width: $stick-size;
    height: $stick-size;
}

.right-stick {
    z-index: 0;
    position: absolute;
    right: $margin-width;
    bottom: $margin-width;
    width: $stick-size;
    height: $stick-size;
}

.left-stick-trim-horizontal {
    position: absolute;
    left: 2 * $margin-width;
    bottom: $margin-width + $stick-size + $margin-width;

    opacity: 0.2;

    & ::v-deep .v-slider {
        margin: 0;
        width: $stick-size - 2 * $margin-width;
    }
}

.left-stick-trim-vertical {
    position: absolute;
    left: $margin-width + $stick-size + $margin-width;
    bottom: 2 * $margin-width;
    width: 32px;

    opacity: 0.2;

    & ::v-deep .v-slider {
        margin: 0;
        height: $stick-size - 2 * $margin-width;
    }
}

.right-stick-trim-horizontal {
    position: absolute;
    right: 2 * $margin-width;
    bottom: $margin-width + $stick-size + $margin-width;

    opacity: 0.2;

    & ::v-deep .v-slider {
        margin: 0;
        width: $stick-size - 2 * $margin-width;
    }
}

.right-stick-trim-vertical {
    position: absolute;
    right: $margin-width + $stick-size + $margin-width;
    bottom: 2 * $margin-width;
    width: 32px;

    opacity: 0.2;

    & ::v-deep .v-slider {
        margin: 0;
        height: $stick-size - 2 * $margin-width;
    }
}

</style>
