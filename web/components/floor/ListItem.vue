<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { type IToolsListItem } from "~/types/tools";
import config from '~/config';

interface Props {
  idx: number;
  item: IToolsListItem;
}
let intersectionObserver: IntersectionObserver;
const props = defineProps<Props>();

onMounted(() => {
  if (!import.meta.env.SSR) {
    const el = document.querySelector(
      `.p-${props.item.category_id}-${props.idx}-${props.item.tid}`
    );
    intersectionObserver = new IntersectionObserver((entries) => {
      if (entries[0].intersectionRatio > 0 && el?.getAttribute('load') !== '1') {
        el?.setAttribute('load', '1');
        (el as HTMLImageElement).src = `${config.CDN_URL}/icon/${props.item.icon}`;
      }
    });
    // 开始监听
    intersectionObserver.observe(el as Element);
  }
});

onBeforeUnmount(() => {
  intersectionObserver.disconnect();
});
</script>

<template>
  <div class="floor-list-item">
    <ATooltip :content="decodeURIComponent(item.describe)">
      <a :href="`/tool/${item.tid}`" target="_blank">
        <div class="head">
          <img
            :class="`p-${item.category_id}-${idx}-${item.tid}`"
            :src="`${config.CDN_URL}/default.png`"
            :alt="decodeURIComponent(item.title)"
          />
          <h3>{{ decodeURIComponent(item.title) }}</h3>
        </div>
        <p>{{ decodeURIComponent(item.describe) }}</p>
      </a>
    </ATooltip>
  </div>
</template>

<style lang="scss" scoped>
.floor-list-item {
  padding-right: 16px;

  a {
    display: block;
    padding: 20px;
    border-radius: 8px;
    box-sizing: border-box;
    background-color: #ffffff;
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    margin-bottom: 10px;
    color: #3c3c3c;

    h3 {
      overflow: hidden;
      font-size: 16px;
      line-height: 1;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    img {
      display: block;
      width: 48px;
      height: 48px;
      margin-right: 12px;
      border-radius: 50%;
      border: 1px solid #f5f6fd;
      background-color: #f5f6fd;
    }
  }

  p {
    font-size: 13px;
    color: #8f8f8f;
    line-height: 20px;
    height: 36px;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  a:hover {
    box-shadow: 0 10px 16px 0 rgba(0, 0, 0, 0.1);
    .head {
      h3 {
        color: #504ded;
      }
      img {
        border: 1px solid #e4e5ec;
      }
    }
    p {
      color: #666;
    }
  }
}
</style>
