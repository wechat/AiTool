<script setup lang="ts">
import config from '~/config';

useSeoMeta({
  title: `登录 - ${config.TITLE}`,
  ogTitle: `登录 - ${config.TITLE}`,
  ogDescription: config.TITLE,
  ogImage: `${config.CDN_URL}/default.png`,
  twitterCard: "summary_large_image",
});
</script>

<template>
  <div class="container"><Login type="login" /></div>
</template>

<style lang="scss" scoped>
.container {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 30px 48px;
  min-width: 1280px;
  height: 600px;
  box-sizing: border-box;
}
</style>
