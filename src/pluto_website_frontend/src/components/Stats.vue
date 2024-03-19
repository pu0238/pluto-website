<template>
  <div class="grid text-white sm:max-w-[36rem] ml-auto px-5 sm:px-0">
    <a
      class="text-white mt-20 bg-greenAccent-500/10 border border-greenAccent-500 p-2 rounded-lg"
      target="_blank"
      :href="href + 'stats'"
    >
      <div class="flex">
        <p
          class="font-lato py-2 px-8 rounded-md font-black bg-greenAccent-500 text-xl text-white"
        >
          GET
        </p>
        <p class="my-auto font-lato font-medium pl-4 rounded-md !text-xl text-white">
          /stats
        </p>
      </div>
    </a>
    <div class="grid pt-4">
      <p class="font-josefin">Request URL</p>
      <code>{{ href }}stats</code>
      <template v-if="response">
        <p class="font-josefin">Response body</p>
        <code>
          <pre>{{ response }}</pre>
        </code>
      </template>
      <button
        class="font-josefin px-8 py-2.5 rounded-3xl bg-greenAccent-500 text-xl border-0 hover:bg-greenAccent-600 ease-out duration-300 w-fit ml-auto mt-2"
        @click="getData()"
      >
        Try it
      </button>
    </div>
  </div>
</template>
<script lang="ts">
export default {
  data() {
    return {
      response: undefined as undefined | string,
    };
  },
  computed: {
    href: () => window.location.href,
  },
  methods: {
    async getPathData(url: string) {
      const res = await fetch(url);
      return JSON.stringify(await res.json(), null, 2);
    },
    async getData() {
      this.response = await this.getPathData(this.href + "stats");
    },
  },
  async mounted() {
    await this.getData();
  },
};
</script>

<style scoped>
  code {
    margin: 4px 0;
    background: #27272a;
    padding: 12px;
    border-radius: 8px;
  }
</style>
