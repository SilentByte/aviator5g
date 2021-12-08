<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <v-app>
        <v-app-bar app dark dense
                   elevation="0"
                   class="app-bar">
            <v-toolbar-items>
                <v-btn text>
                    <img width="100%"
                         height="32"
                         src="@/assets/aviator5g.png"
                         alt="Aviator5G" />
                </v-btn>
            </v-toolbar-items>

            <v-chip v-if="app.isConnected" small color="success">
                <strong>CONNECTED: {{ Math.round(app.roundTripLatency.toMillis() / 2) }}ms</strong>
            </v-chip>
            <v-chip v-else small color="error">
                <strong>DISCONNECTED</strong>
            </v-chip>

            <v-spacer />

            <v-btn icon
                   @click="onToggleFullscreen">
                <v-icon>mdi-fullscreen</v-icon>
            </v-btn>

            <v-menu offset-y>
                <template v-slot:activator="{ on, attrs }">
                    <v-btn icon
                           v-bind="attrs"
                           v-on="on">
                        <v-icon>mdi-cogs</v-icon>
                    </v-btn>
                </template>
                <v-list>
                    <v-list-item @click="onResetTrims">
                        <v-list-item-title>Reset Trims</v-list-item-title>
                    </v-list-item>
                    <v-list-item @click="onResetReverses">
                        <v-list-item-title>Reset Reverses</v-list-item-title>
                    </v-list-item>

                    <v-divider />

                    <v-list-item @click="onReverseAilerons">
                        <v-list-item-title>Reverse Ailerons</v-list-item-title>
                    </v-list-item>
                    <v-list-item @click="onReverseElevator">
                        <v-list-item-title>Reverse Elevator</v-list-item-title>
                    </v-list-item>
                    <v-list-item @click="onReverseRudder">
                        <v-list-item-title>Reverse Rudder</v-list-item-title>
                    </v-list-item>
                    <v-list-item @click="onReverseThrottle">
                        <v-list-item-title>Reverse Throttle</v-list-item-title>
                    </v-list-item>

                    <v-divider />

                    <v-list-item @click="onReloadApp">
                        <v-list-item-title>Reload App</v-list-item-title>
                    </v-list-item>
                </v-list>
            </v-menu>
        </v-app-bar>

        <v-main>
            <router-view />
        </v-main>
    </v-app>
</template>

<!--suppress JSMethodCanBeStatic, JSUnusedGlobalSymbols -->
<script lang="ts">

import {
    Component,
    Vue,
} from "vue-property-decorator";

import { getModule } from "vuex-module-decorators";
import { AppModule } from "@/store/app";

@Component
export default class App extends Vue {
    private readonly app = getModule(AppModule);

    private onToggleFullscreen() {
        if(document.fullscreenElement) {
            document.exitFullscreen();
        } else {
            document.documentElement.requestFullscreen({navigationUI: "hide"});
        }
    }

    private onResetTrims() {
        this.app.doUpdateVehicleState({
            aileronsTrim: 0.0,
            elevatorTrim: 0.0,
            rudderTrim: 0.0,
            throttleTrim: 0.0,
        });
    }

    private onResetReverses() {
        this.app.doUpdateVehicleState({
            aileronsReverse: false,
            elevatorReverse: false,
            rudderReverse: false,
            throttleReverse: false,
        });
    }

    private onReverseAilerons() {
        this.app.doUpdateVehicleState({
            aileronsReverse: !this.app.vehicleState.aileronsReverse,
        });
    }

    private onReverseElevator() {
        this.app.doUpdateVehicleState({
            elevatorReverse: !this.app.vehicleState.elevatorReverse,
        });
    }

    private onReverseRudder() {
        this.app.doUpdateVehicleState({
            rudderReverse: !this.app.vehicleState.rudderReverse,
        });
    }

    private onReverseThrottle() {
        this.app.doUpdateVehicleState({
            throttleReverse: !this.app.vehicleState.aileronsReverse,
        });
    }

    private onReloadApp() {
        location.reload();
    }

    mounted(): void {
        this.app.initializeStore();
    }

    beforeDestroy(): void {
        this.app.uninitializeStore();
    }
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

.app-bar {
    background-color: rgba($primary-color, 0.4) !important;
}

</style>
