<template>
  <div>
    <div class="sm:hidden"  
    @click="toggleMenu"
    :class="{
          'flex justify-center py-2 bg-gradient-to-r from-greenAccent-500 from-10% to-greenAccent-600 to-100% mb-6 cursor-pointer': !isMenuVisible,          
        }">
      <button
        class="sm:hidden flex"
        :class="{
          'absolute top-11 right-4': isMenuVisible,          
        }"
      >
        <svg
          v-if="isMenuVisible"
          width="30"
          height="30"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="sticky z-20"
        >
          <path
            d="M20 20L4 4.00003M20 4L4.00002 20"
            stroke="white"
            stroke-width="2"
            stroke-linecap="round"
          />
        </svg>
        <svg
          v-if="!isMenuVisible"
          width="40"
          height="40"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M4 7a1 1 0 0 1 1-1h14a1 1 0 1 1 0 2H5a1 1 0 0 1-1-1zm0 5a1 1 0 0 1 1-1h14a1 1 0 1 1 0 2H5a1 1 0 0 1-1-1zm0 5a1 1 0 0 1 1-1h14a1 1 0 1 1 0 2H5a1 1 0 0 1-1-1z"
            fill="white"
          />
        </svg>
      </button>
    </div>

    <div
      class="menu sticky pl-5 sm:pl-1 sm:h-full h-screen text-center sm:text-left z-20"
      :class="{
        hidden: !isMenuVisible,
        block: isMenuVisible,
        'sm:block': !isMenuVisible,
      }"
      v-html="summaryContent"
    ></div>
    <div
      v-if="isMenuVisible"
      class="bg-darkAccent-600 fixed h-screen w-screen top-0 z-10"
    ></div>
  </div>
</template>
<script lang="ts">
import { disableScroll, enableScroll } from "../utils/scroll";
export default {
  data() {
    return {
      isMenuVisible: false,
    };
  },
  props: {
    summaryContent: {
      type: String,
      required: true,
    },
  },
  methods: {
    toggleMenu() {
      this.isMenuVisible = !this.isMenuVisible;

      if (this.isMenuVisible) {
        document.body.style.overflow = "hidden";
        disableScroll();
        window.scrollTo(0, 0);
      } else {
        document.body.style.overflow = "";
        enableScroll();
      }
    },
  },
};
</script>
