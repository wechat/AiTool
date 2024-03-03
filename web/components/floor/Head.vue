<script setup lang="ts">
interface Props {
  title: string
  link?: string
}
defineProps<Props>()
</script>

<template>
  <div class="floor-head">
    <h2 class="title">{{ title }}</h2>
    <a :href="link" class="more" v-if="link">
      <span>更多</span>
      <span class="iconfont icon-angle-right-b"></span>
    </a>
  </div>
</template>

<style lang="scss" scoped>
.floor-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;

  .title {
    position: relative;
    color: #111;
    font-size: 24px;
    &::after {
      content: '';
      position: absolute;
      left: 0;
      bottom: 4px;
      width: 100%;
      height: 10px;
      background-color: rgba(80, 77, 237, 0.2);
    }
  }
  .more {
    color: #666;
    &:hover {
      color: #000;
    }
  }
}
</style>
