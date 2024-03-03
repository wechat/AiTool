<script setup lang="ts">
import Header from '~/components/Header.vue'
import Top from '~/components/Top.vue'
import Footer from '~/components/Footer.vue'
</script>

<template>
  <div class="layout">
    <Header />
    <slot />
    <Top />
    <Footer />
  </div>
</template>
