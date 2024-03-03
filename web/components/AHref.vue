<script setup lang="ts">
interface Props {
  type?: string; // 0 不需要新开页，1 需要新开页
  link: string; // 跳转链接
}
const props = defineProps<Props>();
const { type, link } = props;

function jump() {
  if (type === "1") {
    window.location.href = link;
  } else {
    window.location.href = link;
  }
}
</script>

<template>
  <div class="ahref" @click="jump">
    <slot></slot>
  </div>
</template>

<style lang="scss" scoped>
.ahref {
  cursor: pointer;
}
</style>
