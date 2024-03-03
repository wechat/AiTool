<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import config from '~/config';

const router = useRoute();
const { path, query } = router;
const { keyword } = query;
const list = ref([
  {
    label: "站内",
    placeholder: "输入任意关键词搜索AI工具",
    link: "/search?keyword=%s%",
    name: "keyword",
  },
  {
    label: "Bing",
    placeholder: "微软搜索",
    link: "https://cn.bing.com/search?q=%s%",
    name: "q",
  },
  {
    label: "AI搜索",
    placeholder: "人工智能搜索",
    link: "https://aoyo.ai/search?q=%s%",
    name: "q",
  },
  {
    label: "Google",
    placeholder: "谷歌搜索",
    link: "https://www.google.com/search?q=%s%",
    name: "q",
  },
  {
    label: "掘金",
    placeholder: "掘金开发者社区",
    link: "https://juejin.cn/search?query=%s%",
    name: "query",
  },
]);
const currentIndex = ref(0);
const handleTab = (index: number) => {
  currentIndex.value = index;
};
</script>

<template>
  <div class="search">
    <div class="search-inner">
      <div class="search-title">{{ config.DOMAIN }} 发现最好的AI工具</div>
      <form
        :action="list[currentIndex].link"
        method="get"
        :target="path === '/search' && currentIndex === 0 ? '' : '_blank'"
      >
        <ul class="search-tab">
          <li
            :class="{ on: index === currentIndex }"
            v-for="(item, index) in list"
            :key="index"
            @click="handleTab(index)"
          >
            {{ item.label }}
          </li>
        </ul>
        <div class="search-box">
          <div class="search-input">
            <input
              type="text"
              :value="keyword"
              :name="list[currentIndex].name"
              :placeholder="list[currentIndex].placeholder"
            />
          </div>
          <button type="submit" class="search-btn">搜索</button>
        </div>
      </form>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.search {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 360px;
  margin-bottom: 26px;
  border-radius: 8px;
  background-color: #e5e5f6;
  &-inner {
    margin: 0 auto;
    padding-bottom: 40px;
    text-align: center;
  }
  &-title {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 52px;
    color: #222;
    font-size: 54px;
    font-weight: bold;
  }
  &-tab {
    display: flex;
    align-items: center;
    justify-content: center;
    padding-right: 80px;
    li {
      padding: 10px 24px 12px 24px;
      border-radius: 8px 8px 0 0;
      font-weight: bold;
      cursor: pointer;
      &.on {
        background-color: #ffffff;
      }
    }
  }
  &-box {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  &-input {
    background-color: #ffffff;
    border-radius: 8px 0 0 8px;
    input {
      display: flex;
      align-items: center;
      padding: 0 20px;
      width: 500px;
      height: 58px;
      font-size: 20px;
      font-weight: bold;
      line-height: 1;
      background-color: #ffffff;
      border-radius: 8px 0 0 8px;
      box-sizing: border-box;
      &::placeholder {
        font-size: 16px;
        line-height: 1;
      }
    }
  }
  &-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 58px;
    color: #ffffff;
    font-size: 16px;
    border: none;
    cursor: pointer;
    border-radius: 0 8px 8px 0;
    background-color: #504ded;
  }
}
</style>
