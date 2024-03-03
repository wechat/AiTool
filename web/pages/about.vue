<script lang="ts" setup>
import config from '~/config';

useSeoMeta({
  title: `关于本站 - ${config.TITLE}`,
  ogTitle: `关于本站 - ${config.TITLE}`,
  ogDescription: config.TITLE,
  ogImage: `${config.CDN_URL}/default.png`,
  twitterCard: "summary_large_image",
});
</script>

<template>
  <div class="container">
    <h2>免责申明</h2>
    <ol class="declare">
      <li>1、本站功能还在完善中，不保证功能完整。</li>
      <li>
        2、本站所有者按现有技术水平所提供的服务可能存在缺陷，对此导致的任何直接或间接损失，本站不承担法律责任。
      </li>
      <li>3、本站所有数据均来自用户提交和网络整理，不保证数据的真实和正确性。</li>
      <li>
        4、如您在使用本站链接跳转到第三方产品造成间接损失，与本站无关，本站只是提供一个免费的工具导航平台。
      </li>
      <li>
        5、因本站数据均来自用户提交或网络，如本站侵犯到您的版权，请与本站联系，我们立马处理。
      </li>
      <li>6、本站主要目的用来学习交流行业技术。</li>
      <li>
        7、最终解释权归本站所有。如您还有疑问请立马<a
          :href="`mailto:${config.MAILBOX}`"
          >与我们联系</a
        >。
      </li>
    </ol>
    <div class="qrcode">
      <a href="https://curl.qcloud.com/ciIuvbH2" target="_blank">
        <img src="../assets/img/tx2.jpg" />
      </a>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  padding: 30px 48px;
  width: 1100px;
  margin: 30px auto;
  background-color: #fff;
  box-sizing: border-box;
  h2 {
    margin-bottom: 15px;
    padding-left: 10px;
    font-size: 18px;
    font-weight: 500;
    line-height: 1;
    border-left: 5px solid #504ded;
  }
  .declare {
    margin-bottom: 30px;
    li {
      font-size: 14px;
      line-height: 1.6;
      a {
        font-size: 14px;
      }
    }
  }
  .protocol {
    margin-bottom: 30px;
    font-size: 14px;
    line-height: 1.6;
  }
  .qrcode {
    a {
      img {
        width: 1000px;
      }
    }
  }
}
</style>
