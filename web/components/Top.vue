<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
const show = ref(false)

const handleScrollTop = () => {
  if (!import.meta.env.SSR) {
    window.scrollTo({
      top: 0,
      behavior: 'smooth'
    })
  }
}

const handleScroll = () => {
  const scrollTop = document.documentElement.scrollTop
  if (scrollTop > 300) {
    show.value = true
  } else {
    show.value = false
  }
}

onMounted(() => {
  if (!import.meta.env.SSR) {
    document.addEventListener('scroll', handleScroll, true)
    document.onresize = () => {
      handleScroll()
    }
  }
})

onUnmounted(() => {
  if (!import.meta.env.SSR) {
    document.removeEventListener('scroll', handleScroll)
  }
})
</script>

<template>
  <div v-if="show" class="gototop" @click="handleScrollTop">
    <span class="iconfont icon-top"></span>
  </div>
</template>

<style lang="scss" scoped>
.gototop {
  position: fixed;
  right: 36px;
  bottom: 36px;
  z-index: 998;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  cursor: pointer;
  border-radius: 8px;
  background-color: rgba(0, 0, 0, 0.1);
  animation: tada 0.5s;
  span {
    color: #999;
    font-size: 26px;
  }
  &:hover {
    background-color: rgba(0, 0, 0, 0.2);
  }
}
@keyframes tada {
  from {
    transform: scale3d(0, 0, 0);
  }
  // 10%,
  // 20% {
  //   transform: scale3d(0.9, 0.9, 0.9) rotate3d(0, 0, 1, -3deg);
  // }
  // 30%,
  // 50%,
  // 70%,
  // 90% {
  //   transform: scale3d(1.1, 1.1, 1.1) rotate3d(0, 0, 1, 3deg);
  // }
  // 40%,
  // 60%,
  // 80% {
  //   transform: scale3d(1.1, 1.1, 1.1) rotate3d(0, 0, 1, -3deg);
  // }
  80% {
    transform: scale3d(1.2, 1.2, 1.2);
  }
  to {
    transform: scale3d(1, 1, 1);
  }
}
</style>
