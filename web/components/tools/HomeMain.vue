<script setup lang="ts">
import FloorHead from "~/components/floor/Head.vue";
import FloorList from "~/components/floor/List.vue";
import Search from "~/components/Search.vue";
import { useMain } from "~/composables/useMain";
const { sidebar } = useMain();
const { hotList, newList, homeList } = useTools();
</script>

<template>
  <div class="main">
    <Search />
    <div class="floor" id="floor0">
      <FloorHead title="热门推荐" link="/hot" />
      <FloorList :list="hotList" />
    </div>
    <div class="floor">
      <FloorHead title="最新推出" link="/new" />
      <FloorList :list="newList" />
    </div>
    <div v-for="(item, index) in sidebar" :key="index">
      <div class="floor" v-if="index !== 0" :id="`floor${index}`">
        <FloorHead
          :title="item.name"
          :link="`/category?category_id=${index}`"
        />
        <FloorList
          :list="homeList[index]"
          v-if="homeList && homeList[index]?.length"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.main {
  flex: 1;
}
.floor {
  margin-bottom: 10px;
}
</style>
