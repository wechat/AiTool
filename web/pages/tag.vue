<script setup lang="ts">
import config from '~/config';

const { tagsList, fetchTagsListApi } = useTools();

const router = useRoute();
const { tag } = router.query;

useSeoMeta({
  title: `${decodeURIComponent(String(tag))} - ${config.TITLE}`,
  ogTitle: `${decodeURIComponent(String(tag))} - ${config.TITLE}`,
  ogDescription: config.TITLE,
  ogImage: `${config.CDN_URL}/default.png`,
  twitterCard: "summary_large_image",
});

onServerPrefetch(async () => {
  await fetchTagsListApi(String(tag), 0, 500);
});
</script>

<template>
  <div class="container">
    <Sidebar />
    <div class="main">
      <TopTag :title="String(tag)" v-if="tag" />
      <FloorList :list="tagsList" v-if="tagsList.length" />
      <AEmpty v-else class="empty" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  display: flex;
  justify-content: flex-start;
  padding: 30px 48px;
  min-width: 1280px;
  box-sizing: border-box;
  .main {
    flex: 1;
  }
  .empty {
    padding: 120px 0;
    font-size: 26px;

    &:deep(.arco-icon) {
      font-size: 52px;
    }
  }
}
</style>
