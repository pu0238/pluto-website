<template>
  <div class="relative">
    <a href="/" class="logo sticky z-20">Pu</a>
    <div class="absolute flex right-0 top-8">
      <button class="flex pr-6 sm:pr-4" @click="toggleSearch">
        <div>
          <svg
            viewBox="0 0 9 9"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            style="transform: translate(24px, 6px)"
            class="fill-white sm:fill-gray-300/80 w-[17px] h-[17px] sm:h-3.5 sm:w-3.5"
          >
            <path
              d="M8.85215 8.14238L7.1527 6.44792C7.70101 5.74938 7.99853 4.88675 7.99742 3.99871C7.99742 3.20784 7.7629 2.43473 7.32352 1.77715C6.88414 1.11956 6.25962 0.607038 5.52895 0.304386C4.79828 0.00173257 3.99428 -0.0774552 3.2186 0.0768359C2.44293 0.231127 1.73043 0.611967 1.1712 1.1712C0.611967 1.73043 0.231127 2.44293 0.0768359 3.2186C-0.0774552 3.99428 0.00173257 4.79828 0.304386 5.52895C0.607038 6.25962 1.11956 6.88414 1.77715 7.32352C2.43473 7.7629 3.20784 7.99742 3.99871 7.99742C4.88675 7.99853 5.74938 7.70101 6.44792 7.1527L8.14238 8.85215C8.18884 8.899 8.24413 8.93618 8.30504 8.96156C8.36595 8.98694 8.43128 9 8.49726 9C8.56325 9 8.62858 8.98694 8.68949 8.96156C8.7504 8.93618 8.80568 8.899 8.85215 8.85215C8.899 8.80568 8.93618 8.7504 8.96156 8.68949C8.98694 8.62858 9 8.56325 9 8.49726C9 8.43128 8.98694 8.36595 8.96156 8.30504C8.93618 8.24413 8.899 8.18884 8.85215 8.14238ZM0.99968 3.99871C0.99968 3.40556 1.17557 2.82573 1.50511 2.33254C1.83465 1.83935 2.30303 1.45496 2.85103 1.22797C3.39903 1.00098 4.00204 0.941587 4.5838 1.05731C5.16555 1.17302 5.69993 1.45865 6.11935 1.87808C6.53877 2.2975 6.8244 2.83188 6.94012 3.41363C7.05584 3.99539 6.99645 4.59839 6.76946 5.14639C6.54247 5.6944 6.15808 6.16278 5.66489 6.49232C5.1717 6.82186 4.59187 6.99775 3.99871 6.99775C3.20332 6.99775 2.4405 6.68178 1.87808 6.11935C1.31565 5.55692 0.99968 4.79411 0.99968 3.99871Z"
            />
          </svg>
        </div>
        <p class="font-lato !text-sm border-b p-0.5 border-[#9e9e9e] pl-7 bg-transparent sm:pr-28 pr-10 text-left hidden sm:block hover:border-greenAccent-500 ease-out duration-300">
          Search
        </p>
      </button>
      <a
        href="https://github.com/pu0238/pluto"
        target="_blank"
        class="button"
        style="margin-left: 10px; margin-top: 5px"
      >
        <img src="/git.svg" />
      </a>
      <a
        href="https://t5mcf-cqaaa-aaaag-qcjna-cai.raw.icp0.io/"
        class="button"
        target="_blank"
        style="margin-left: 10px; margin-top: 5px"
      >
        <img src="/globe.svg" />
      </a>
      <a
        href="https://discord.gg/HYvVN9HK2s"
        class="button"
        target="_blank"
        style="margin-left: 10px; margin-top: 5px"
      >
        <img src="/discord.svg" />
      </a>
    </div>
  </div>
  <Search
    v-if="isSearchVisible"
    :searchData="searchData"
    @closeSearch="toggleSearch"
  />
</template>
<script lang="ts">
import Search from "../components/Search.vue";
import { disableScroll, enableScroll } from "../utils/scroll";
import type { SearchBy } from "../utils/types/SearchBy";
export default {
  data() {
    return {
      isSearchVisible: false,
    };
  },
  components: {
    Search,
  },
  props: {
    searchData: {
      type: Array<SearchBy>,
      required: true,
    },
  },
  methods: {
    toggleSearch() {
      this.isSearchVisible = !this.isSearchVisible;

      if (this.isSearchVisible) {
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
