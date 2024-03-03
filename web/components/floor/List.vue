<script setup lang="ts">
import { type IToolsListItem } from '~/types/tools'
import ListItem from '~/components/floor/ListItem.vue'

interface Props {
  list: IToolsListItem[]
}
defineProps<Props>()
</script>

<template>
  <ul class="floor-list">
    <li v-for="(item, index) in list" :key="index">
      <ListItem :item="item" :idx="index" />
    </li>
  </ul>
</template>

<style lang="scss" scoped>
.floor-list {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  flex-wrap: wrap;
  li {
    flex-shrink: 0;
    width: 16.6666%;
    margin: 0 0 16px 0;

    &:nth-child(6n) {
      .floor-list-item {
        padding-right: 0;
      }
    }
  }
}
</style>
