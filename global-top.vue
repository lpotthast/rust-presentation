<script setup lang="ts">
import { computed } from 'vue'
import { useNav } from '@slidev/client'

const { slides, currentPage, currentLayout, total } = useNav()

// chapter = title of the last section slide up to the current slide
const chapter = computed(() => {
  let title = ''
  for (const route of slides.value) {
    if (route.no > currentPage.value)
      break
    if (route.meta.slide.frontmatter.layout === 'section')
      title = route.meta.slide.title ?? ''
  }
  return title
})
</script>

<template>
  <footer
    v-if="!['cover', 'section', 'center'].includes(currentLayout)"
    class="absolute bottom-3 left-8 right-8 flex justify-between text-xs"
    style="color: var(--mp-dim); font-family: var(--slidev-code-font-family)"
  >
    <span class="flex items-center gap-2">
      <img src="/ferris.svg" alt="" class="h-3" />Rust - An Introduction<template v-if="chapter"> / {{ chapter }}</template>
    </span>
    <span>{{ currentPage }} / {{ total }}</span>
  </footer>
</template>
