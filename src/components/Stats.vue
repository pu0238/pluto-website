<template>
  <div class="grid text-white sm:max-w-[36rem] ml-auto">
    <a
      class="text-white mt-20 path-container p-2 rounded-lg"
      target="_blank"
      :href="href + 'stats'"
    >
      <div class="flex">
        <p class="get">GET</p>
        <p class="my-auto path">/stats</p>
      </div>
    </a>
    <div class="grid pt-4">
      <p class="title">Request URL</p>
      <code>{{ href }}stats</code>
      <template v-if="response">
        <p class="title">Response body</p>
        <code>
          <pre>{{ response }}</pre>
        </code>
      </template>
      <button
        class="ease-out duration-300 w-fit ml-auto mt-2"
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
