<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <v-app>
        <v-app-bar app dark dense
                   elevation="0"
                   color="primary">
            <v-toolbar-items>
                <v-btn text>
                    <img width="100%"
                         height="32"
                         src="@/assets/aviator5g.png"
                         alt="Aviator5G" />
                </v-btn>
            </v-toolbar-items>

            <v-spacer />

            <v-btn icon>
                <v-icon>mdi-fullscreen</v-icon>
            </v-btn>

            <v-btn icon>
                <v-icon>mdi-cogs</v-icon>
            </v-btn>
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

    mounted(): void {
        this.app.initializeStore();
    }

    beforeDestroy(): void {
        this.app.uninitializeStore();
    }
}

</script>
