<script setup lang="ts">
const { params } = useRoute();
const { hfSidebar, phSidebar } = useMain();
const { type, time } = params;
interface Props {
  from: string;
}
const props = defineProps<Props>();
const sidebar = ref(props.from === "ph" ? phSidebar : hfSidebar);
</script>

<template>
  <nav class="sidebar">
    <ul>
      <li
        :class="{ on: item.actived }"
        v-for="(item, index) in sidebar"
        :key="index"
      >
        <a><span :class="`iconfont ${item.icon}`"></span>{{ item.name }}</a>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
.sidebar {
  position: relative;
  flex-shrink: 0;
  margin-right: 24px;
  width: 180px;
  border-radius: 8px;

  ul {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 10;
    width: 180px;

    &.top {
      position: fixed;
      top: 110px;
      left: 48px;
      bottom: auto;
    }

    &.bottom {
      position: absolute;
      top: auto;
      left: 0;
      bottom: 28px;
      transition: bottom 0.4s;
      -webkit-transition: bottom 0.4s;
    }
  }

  li {
    background-color: #fff;
    a {
      display: block;
      padding: 14px 20px;
      color: #666;
      font-size: 15px;
      font-weight: bold;
      cursor: pointer;

      span {
        margin-right: 10px;
        font-size: 18px;
      }
    }

    a:hover {
      background-color: rgba(80, 77, 237, 0.1);
    }

    &.on a {
      color: #fff;
      border-radius: 8px;
      background-color: #504ded;
    }
  }
}
</style>
