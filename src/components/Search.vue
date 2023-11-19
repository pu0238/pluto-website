<template>
  <div
    class="bg-zinc-800 xl:w-2/5 lg:w-3/5 md:w-3/5 sm:w-4/5 w-5/6 max-h-[350px] fixed mx-auto sm:inset-x-0 top-15 z-40 overflow-auto"
  >
    <div class="relative">
      <svg
        width="14"
        height="14"
        viewBox="0 0 9 9"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        style="transform: translate(16px, 20px)"
      >
        <path
          d="M8.85215 8.14238L7.1527 6.44792C7.70101 5.74938 7.99853 4.88675 7.99742 3.99871C7.99742 3.20784 7.7629 2.43473 7.32352 1.77715C6.88414 1.11956 6.25962 0.607038 5.52895 0.304386C4.79828 0.00173257 3.99428 -0.0774552 3.2186 0.0768359C2.44293 0.231127 1.73043 0.611967 1.1712 1.1712C0.611967 1.73043 0.231127 2.44293 0.0768359 3.2186C-0.0774552 3.99428 0.00173257 4.79828 0.304386 5.52895C0.607038 6.25962 1.11956 6.88414 1.77715 7.32352C2.43473 7.7629 3.20784 7.99742 3.99871 7.99742C4.88675 7.99853 5.74938 7.70101 6.44792 7.1527L8.14238 8.85215C8.18884 8.899 8.24413 8.93618 8.30504 8.96156C8.36595 8.98694 8.43128 9 8.49726 9C8.56325 9 8.62858 8.98694 8.68949 8.96156C8.7504 8.93618 8.80568 8.899 8.85215 8.85215C8.899 8.80568 8.93618 8.7504 8.96156 8.68949C8.98694 8.62858 9 8.56325 9 8.49726C9 8.43128 8.98694 8.36595 8.96156 8.30504C8.93618 8.24413 8.899 8.18884 8.85215 8.14238ZM0.99968 3.99871C0.99968 3.40556 1.17557 2.82573 1.50511 2.33254C1.83465 1.83935 2.30303 1.45496 2.85103 1.22797C3.39903 1.00098 4.00204 0.941587 4.5838 1.05731C5.16555 1.17302 5.69993 1.45865 6.11935 1.87808C6.53877 2.2975 6.8244 2.83188 6.94012 3.41363C7.05584 3.99539 6.99645 4.59839 6.76946 5.14639C6.54247 5.6944 6.15808 6.16278 5.66489 6.49232C5.1717 6.82186 4.59187 6.99775 3.99871 6.99775C3.20332 6.99775 2.4405 6.68178 1.87808 6.11935C1.31565 5.55692 0.99968 4.79411 0.99968 3.99871Z"
          fill="#60f78e"
        />
      </svg>
    </div>
    <input
      v-model="searchBy"
      @input="showSuggestions"
      placeholder="Search"
      class="pl-9 w-full pr-14 bg-transparent text-white outline-none"
      contenteditable="true"
    />
    <button
      class="right-7 top-5 absolute"
      @click="removeInput"
      v-if="InputEmpty"
    >
      <svg
        fill="white"
        height="12px"
        width="12px"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        viewBox="0 0 460.775 460.775"
        xml:space="preserve"
        class="hover:fill-greenAccent-500"
      >
        <path
          d="M285.08,230.397L456.218,59.27c6.076-6.077,6.076-15.911,0-21.986L423.511,4.565c-2.913-2.911-6.866-4.55-10.992-4.55
	c-4.127,0-8.08,1.639-10.993,4.55l-171.138,171.14L59.25,4.565c-2.913-2.911-6.866-4.55-10.993-4.55
	c-4.126,0-8.08,1.639-10.992,4.55L4.558,37.284c-6.077,6.075-6.077,15.909,0,21.986l171.138,171.128L4.575,401.505
	c-6.074,6.077-6.074,15.911,0,21.986l32.709,32.719c2.911,2.911,6.865,4.55,10.992,4.55c4.127,0,8.08-1.639,10.994-4.55
	l171.117-171.12l171.118,171.12c2.913,2.911,6.866,4.55,10.993,4.55c4.128,0,8.081-1.639,10.992-4.55l32.709-32.719
	c6.074-6.075,6.074-15.909,0-21.986L285.08,230.397z"
        />
      </svg>
    </button>
    <div class="py-5 px-5">
      <div
        v-for="{ title, headings, url} in suggestions"
        :key="title"
        v-if="searchBy !== ''"
      >
        <p
          class="text-greenAccent-500 w-full py-1.5 text-left font-medium rounded-md"
        >
          {{ title }} 
        </p>
        <div v-for="{ text, slug } in headings" :key="slug" class="py-1">
          <a
            :href="`${url}#${slug}`"
            v-if="searchBy !== ''"
            class="text-white hover:text-black flex text-left w-full py-3 px-2 shadow-sm bg-zinc-700/25 hover:bg-greenAccent-500 shadow-zinc-900 rounded text-sm relative group/item"
          >
            <div class="pt-0.5 pr-1">
              <svg
                fill="white"
                height="15"
                viewBox="0 0 24 24"
                width="15"
                xmlns="http://www.w3.org/2000/svg"
              >
                <g stroke="#000" stroke-linejoin="round" stroke-width="2">
                  <g stroke-linecap="round">
                    <path
                      d="m4 4v16c0 1.1046.89543 2 2 2h12c1.1046 0 2-.8954 2-2v-11.65838c0-.53822-.2169-1.05373-.6018-1.43001l-4.4403-4.34162c-.3737-.3654-.8755-.56999-1.3982-.56999h-7.5597c-1.10457 0-2 .89543-2 2z"
                    />
                    <path d="m9 13h6" />
                    <path d="m9 17h3" />
                  </g>
                  <path d="m14 2v4c0 1.10457.8954 2 2 2h4" />
                </g>
              </svg>
            </div>
            {{ text }}
            <div class="group-hover/item:block hidden absolute right-5">
              <svg
                fill="none"
                height="20"
                viewBox="0 0 24 24"
                width="20"
                xmlns="http://www.w3.org/2000/svg"
              >
                <g
                  stroke="black"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                >
                  <path d="m18 6-12 12" />
                  <path d="m8 6h10v10" />
                </g>
              </svg>
            </div>
            
          </a>
        </div>
      </div>

      <p
        class="py-5 text-center"
        v-if="suggestions.length === 0 && searchBy !== ''"
      >
      No results for  <p class="text-white break-words">"{{ searchBy }}"</p>
      </p>
      <p class="py-5 text-center" v-else-if="searchBy === ''">
        No recent searches
      </p>
    </div>
  </div>
  <div
    class="z-30 fixed h-screen w-screen -ml-8 -mt-20 bg-black/30 backdrop-blur-sm"
    @click="$emit('closeSearch')"
  ></div>
</template>
<script lang="ts">
import type { SearchBy } from "../utils/types/SearchBy";
export default {
  data() {
    return {
      searchBy: "",
      InputEmpty: false,
    };
  },
  props: {
    searchData: {
      type: Array<SearchBy>,
      required: true,
    },
  },
  computed: {
    suggestions(): SearchBy[] {
      const searchBy = this.searchBy.toLowerCase();
      const filteredArticles = this.searchData.filter(({ headings }) =>
        headings.some((heading) =>
          heading.text.toLowerCase().includes(searchBy)
        )
      );
      return filteredArticles.map((data) => {
        const headings = data.headings.filter(({ text }) =>
          text.toLowerCase().includes(searchBy)
        );
        return { ...data, headings };
      });
    },
  },
  methods: {
    showSuggestions() {
      if (this.searchBy === "") {
        this.InputEmpty = false;
      } else {
        this.InputEmpty = true;
      }
    },
    removeInput() {
      if (this.searchBy === "") return;
      this.searchBy = "";
      this.InputEmpty = false;
    },
  },
  emits: ["closeSearch"],
};
</script>
