<script setup lang="ts">
import config from '~/config';
import DetailMain from "~/components/tools/DetailMain.vue";

const router = useRoute();
const { detail, fetchToolDetailApi } = useTools();
const { tid } = router.params;

onServerPrefetch(async () => {
  await fetchToolDetailApi(tid as string);

  useSeoMeta({
    title: `${decodeURIComponent(String(detail.value?.title))} - ${config.TITLE}`,
    ogTitle: `${decodeURIComponent(String(detail.value?.title))} - ${config.TITLE}`,
    ogDescription: decodeURIComponent(String(detail.value?.describe)),
    ogImage: `${config.CDN_URL}/default.png`,
    twitterCard: "summary_large_image",
  });
});
</script>

<template>
  <div class="container">
    <Sidebar />
    <DetailMain />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  justify-content: flex-start;
  padding: 30px 48px;
  min-width: 1280px;
  box-sizing: border-box;
}
</style>
