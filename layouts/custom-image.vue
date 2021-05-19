<script setup lang="ts">
import { computed, defineProps } from 'vue'

const props = defineProps({
  image: {
    type: String,
  },
  class: {
    type: String,
  },
  size: {
    type: String,
    default: 'cover',
  }
})
const style = computed(() => ({
  background: `url(${props.image}) no-repeat`,
  backgroundSize: props.size,
  backgroundPosition: 'center',
}))
</script>

<template>
  <div class="w-full h-full">
    <div class="w-full h-full" :style="style" />
  </div>
</template>
