<script setup lang="ts">
import HomeMain from '~/components/tools/HomeMain.vue'
const {fetchHotListApi, fetchNewListApi, fetchHomeListApi} = useTools()

onServerPrefetch(async () => {
  await fetchHotListApi()
  await fetchNewListApi()
  await fetchHomeListApi()
})
</script>

<template>
  <div class="container">
    <Sidebar />
    <HomeMain />
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
