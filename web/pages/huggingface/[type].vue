<script setup lang="ts"></script>

<template>
  <div class="container">
    <SimpleSidebar from="hf" />
    <div class="main">
      <div>huggingface</div>
      <AEmpty class="empty" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  display: flex;
  justify-content: flex-start;
  padding: 30px 48px;
  min-width: 1280px;
  min-height: 600px;
  box-sizing: border-box;

  .empty {
    padding: 120px 0;
    font-size: 26px;

    &:deep(.arco-icon) {
      font-size: 52px;
    }
  }
}
</style>
