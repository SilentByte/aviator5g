<!--
    AVIATOR 5G SYSTEM
    Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 -->
<template>
    <div>
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

@Component({
    components: {
        VirtualJoystick,
    },
})
export default class HomeView extends Vue {
    private readonly app = getModule(AppModule);

    private onMoveLeftStick(e: IVirtualJoystickEvent) {
        console.log(e.vector);
    }

    private onMoveRightStick(e: IVirtualJoystickEvent) {
        console.log(e.vector);
    }
}

</script>

<style lang="scss" scoped>

@import "~@/styles/variables.scss";

.left-stick {
    position: absolute;
    left: 0;
    bottom: 0;
    width: 200px;
    height: 200px;
}

.right-stick {
    position: absolute;
    right: 0;
    bottom: 0;
    width: 200px;
    height: 200px;
}

</style>
