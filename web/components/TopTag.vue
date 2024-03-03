<script setup lang="ts">
interface Props {
  title: string
}
defineProps<Props>()
</script>

<template>
  <div class="top-tag">#{{ title }}#</div>
</template>

<style lang="scss" scoped>
.top-tag {
  margin-bottom: 26px;
  padding: 30px 0;
  text-align: center;
  color: #222;
  font-size: 38px;
  font-weight: bold;
  border-radius: 8px;
  background-color: rgba(80, 77, 237, 0.1);
}
</style>
