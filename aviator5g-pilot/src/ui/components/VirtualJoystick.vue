<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <div>
        <div ref="joystick" class="joystick" />
    </div>
</template>

<!--suppress JSMethodCanBeStatic, JSUnusedGlobalSymbols -->
<script lang="ts">

import nipplejs, {
    EventData,
    JoystickManager,
    JoystickOutputData,
} from "nipplejs";

import {
    Component, Prop,
    Vue,
} from "vue-property-decorator";

import { getModule } from "vuex-module-decorators";
import { AppModule } from "@/store/app";

export interface IVirtualJoystickEvent {
    eventData: EventData;
    joystickData?: JoystickOutputData;
    vector: {
        x: number;
        y: number;
    };
}

interface IVector {
    x: number;
    y: number;
}

@Component
export default class VirtualJoystick extends Vue {
    @Prop({type: Number, required: true}) size!: number;
    @Prop({type: Boolean, default: true}) restX!: boolean;
    @Prop({type: Boolean, default: true}) restY!: boolean;

    private readonly app = getModule(AppModule);
    private manager!: JoystickManager;
    private lastVector: IVector = {x: 0, y: 0};

    mounted(): void {
        this.manager = nipplejs.create({
            zone: this.$refs.joystick as any,
            color: this.$vuetify.theme.currentTheme.accent?.toString() || "red",
            mode: "static",
            position: {left: "50%", top: "50%"},
            size: this.size,
            restJoystick: {x: this.restX, y: this.restY},
            maxNumberOfNipples: 1,
        });

        (this.manager as any).get().on("move", (e: EventData, data: JoystickOutputData) => {
            this.lastVector = data.vector;
            this.$emit("move", {
                eventData: e,
                joystickData: data,
                vector: data.vector,
            });
        });

        (this.manager as any).get().on("end", (e: EventData) => {
            this.$emit("move", {
                eventData: e,
                vector: {
                    x: this.restX ? 0 : this.lastVector.x,
                    y: this.restY ? 0 : this.lastVector.y,
                },
            });
        });
    }

    beforeDestroy(): void {
        this.manager.destroy();
    }
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

.joystick {
    position: static;
}

</style>


