<script setup lang="ts">
import config from '~/config';

const { sidebar } = useMain();
const { categoryList, fetchCategoryListApi } = useTools();

const router = useRoute();
const { category_id } = router.query;

useSeoMeta({
  title: `${sidebar.value[Number(category_id)].name} - ${config.TITLE}`,
  ogTitle: `${sidebar.value[Number(category_id)].name} - ${config.TITLE}`,
  ogDescription: config.TITLE,
  ogImage: `${config.CDN_URL}/default.png`,
  twitterCard: "summary_large_image",
});

onServerPrefetch(async () => {
  await fetchCategoryListApi(Number(category_id), 0, 500);
});
</script>

<template>
  <div class="container">
    <Sidebar />
    <div class="main">
      <TopTag :title="sidebar[Number(category_id)].name" v-if="sidebar[Number(category_id)].name" />
      <FloorList :list="categoryList" v-if="categoryList.length" />
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
