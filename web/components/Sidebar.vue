<script setup lang="ts">
const { sidebar, anchorPosition, setActived } = useMain();

const router = useRoute();
const { path, query } = router;
const { category_id } = query;

interface Props {
  title?: string;
}
defineProps<Props>();
const classname = ref("");
const clientHeight = ref("");
const floorOffsetTop = ref<number[]>([]);

if (category_id) {
  setActived(Number(category_id));
} else {
  setActived(0);
}

const handleClick = (index: number) => {
  if (!import.meta.env.SSR) {
    if (path === "/") {
      anchorPosition(`#floor${index}`);
      setActived(index);
    } else {
      if (index === 0) {
        location.href = `/hot`;
      } else {
        location.href = `/category?category_id=${index}`;
      }
    }
  }
};

const debounce = (fn: (...args: number[]) => void, delay = 200) => {
  let timer: ReturnType<typeof setTimeout> | null;
  return (...args: number[]) => {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
};

const changeActived = debounce((scrollTop: number) => {
  if (floorOffsetTop.value?.length > 0) {
    for (let i = 0; i < floorOffsetTop.value.length; i++) {
      const item = floorOffsetTop.value[i];
      if (scrollTop > item - 120) {
        setActived(i);
      } else if (scrollTop <= floorOffsetTop.value[0]) {
        setActived(0);
      }
    }
  }
}, 100);

const handleScroll = () => {
  const scrollTop =
    document.documentElement.scrollTop || document.body.scrollTop;
  const scrollHeight =
    document.documentElement.scrollHeight || document.body.scrollHeight;
  if (scrollTop > 0) {
    classname.value = "top";
    if (scrollTop >= scrollHeight - 1212) {
      classname.value = "bottom";
    }
  } else {
    classname.value = "";
  }
  changeActived(scrollTop);
};

onMounted(() => {
  if (!import.meta.env.SSR) {
    if (path === "/") {
      handleScroll();
      clientHeight.value = "auto";
      document.addEventListener("scroll", handleScroll, true);
      document.onresize = () => {
        handleScroll();
      };
      if (sidebar.value.length > 0) {
        const arr: number[] = [];
        sidebar.value.map((_, index: number) => {
          const element: HTMLAnchorElement | null = document.querySelector(
            `#floor${index}`
          );
          element && arr.push(element.offsetTop);
        });
        floorOffsetTop.value = arr;
      }
    } else {
      const h = document.querySelector(".sidebar ul")?.clientHeight;
      clientHeight.value = `${Number(h)}px`;
    }
  }
});

onUnmounted(() => {
  if (!import.meta.env.SSR && path === "/") {
    document.removeEventListener("scroll", handleScroll);
  }
});
</script>

<template>
  <nav class="sidebar" :style="`height: ${clientHeight}`">
    <ul :class="classname">
      <li
        :class="{ on: item.actived }"
        v-for="(item, index) in sidebar"
        :key="index"
      >
        <a @click="handleClick(index)"
          ><span :class="`iconfont ${item.icon}`"></span>{{ item.name }}</a
        >
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
