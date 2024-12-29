<template>
  <img
    :src="path"
    :width="size"
    :height="size"
    :style="{ fill: color }"
    class="inline-block align-middle"
  />
</template>

<script setup>
import { defineProps } from "vue";

defineProps({
  /**
   * The file path for the external SVG.
   */
  path: {
    type: String,
    required: true,
  },
  /**
   * The size of the icon (width and height).
   */
  size: {
    type: [String, Number],
    default: 24,
  },
  /**
   * The color of the icon.
   */
  color: {
    type: String,
    default: "currentColor",
  },
});
</script>

