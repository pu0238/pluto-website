<template>
  <div class="sticky hidden md:block ">
    <div class="pr-10 border-l-[1px] h-fit">
      <div
        class="relative text-sm"
        v-for="heading in headings"
        :key="heading.slug"
        :class="{
          'square active-heading': activeHeadingSlug === heading.slug,
          square: activeHeadingSlug !== heading.slug,
        }"
      >
        <p
          :style="{ marginLeft: `${heading.depth * 10}px` }"
          class="text-white cursor-pointer"
          @click="() => goToHeading(heading)"
        >
          {{ heading.text }}
        </p>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
export default {
  data() {
    return {
      activeHeadingSlug: this.getActiveHeading(),
    };
  },
  watch: {
    activeHeadingSlug(newSlug) {
      this.setSlugHistory(newSlug);
    },
  },
  props: {
    headings: {
      type: Array<{ depth: number; slug: string; text: string }>,
      required: true,
    },
  },
  methods: {
    getActiveHeading() {
      let closestPosition = Infinity;
      let closestHeading = null;
      this.headings.forEach((heading) => {
        const headingBySlug = document.getElementById(heading.slug);
        if (!headingBySlug) return;

        const rect = headingBySlug.getBoundingClientRect();
        const topOffset = rect.top + rect.height;

        if (topOffset < 0) return;

        if (topOffset < closestPosition) {
          closestPosition = topOffset;
          closestHeading = heading.slug;
        }
      });
      return closestHeading;
    },
    goToHeading(heading: { slug: string }) {
      const headingElement = document.getElementById(heading.slug);
      if (!headingElement) return;
      headingElement.scrollIntoView({ behavior: "smooth" });
    },
    setSlugHistory(newSlug: string) {
      window.history.pushState(
        {},
        "",
        `${window.location.origin}${window.location.pathname}#${newSlug}`
      );
    },
  },
  mounted() {
    addEventListener("scroll", () => {
      this.activeHeadingSlug = this.getActiveHeading();
    });
  },
};
</script>
<style scoped>
.square::before {
  content: "";
  height: 9px;
  width: 9px;
  position: absolute;
  background-color: white;
  left: -5px;
  top: 6px;
  transform: rotate(45deg);
}
.active-heading.square::before {
  background-color: var(--accent);
}
</style>
