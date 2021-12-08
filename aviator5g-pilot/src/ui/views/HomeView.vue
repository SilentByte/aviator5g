<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <v-container fill-height class="align-start" style="margin-bottom: 80px">
        <v-row dense>
            <v-col cols="12">
                Aviator 5G
            </v-col>
        </v-row>
    </v-container>
</template>

<!--suppress JSMethodCanBeStatic -->
<script lang="ts">

import {
    Component,
    Vue,
} from "vue-property-decorator";

import { getModule } from "vuex-module-decorators";
import { AppModule } from "@/store/app";

@Component
export default class HomeView extends Vue {
    private readonly app = getModule(AppModule);
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

</style>
