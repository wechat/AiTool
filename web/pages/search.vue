<script setup lang="ts">
import config from '~/config';

const { searchList, fetchSearchListApi } = useSearch();

const router = useRoute();
const { keyword } = router.query;

if (keyword) {
  useSeoMeta({
    title: `${keyword} - ${config.TITLE}`,
    ogTitle: `${keyword} - ${config.TITLE}`,
    ogDescription: config.TITLE,
    ogImage: `${config.CDN_URL}/default.png`,
    twitterCard: "summary_large_image",
  });
}

onServerPrefetch(async () => {
  await fetchSearchListApi(String(keyword));
});
</script>

<template>
  <div class="container">
    <Search />
    <FloorList :list="searchList" v-if="searchList.length" />
    <AEmpty v-else class="empty" />
  </div>
</template>

<style lang="scss" scoped>
.container {
  padding: 30px 48px;
  min-width: 1280px;
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
