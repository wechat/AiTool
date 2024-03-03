<script setup lang="ts">
import config from '~/config';

interface Props {
  title?: string;
}
defineProps<Props>();
</script>

<template>
  <div class="footer">
    <footer>
      <div class="contact">
        <a class="logo" href="/">
          <span class="iconfont icon-logo"></span>
          <h1>{{ config.DOMAIN }}</h1>
        </a>
        <ul>
          <li>
            <a class="mailbox" :href="`mailto:${config.MAILBOX}`" target="_blank"
              ><span class="iconfont icon-users-alt"></span>交流合作</a
            >
          </li>
          <li>
            <a class="mailbox" :href="`mailto:${config.MAILBOX}`" target="_blank"
              ><span class="iconfont icon-envelope"></span>联系我们</a
            >
          </li>
          <li>
            Copyright © 2024 {{ config.DOMAIN }}
            <a href="https://beian.miit.gov.cn" target="_blank">{{
              config.COPYRIGHT
            }}</a>
          </li>
        </ul>
      </div>
      <div class="link">
        <div class="title">友情链接</div>
        <ul>
          <li><a href="https://tongyi.aliyun.com/qianwen/" target="_blank">通义千问</a></li>
          <li><a href="https://yiyan.baidu.com/" target="_blank">文心一言</a></li>
          <li><a :href="`https://chatglm.cn/?fr=${config.DOMAIN}`" target="_blank">智谱清言</a></li>
        </ul>
      </div>
      <div class="link">
        <div class="title">关于本站</div>
        <ul>
          <li><a href="/about" target="_blank">免责声明</a></li>
          <li><a :href="config.SUBMIT_URL" target="_blank">提交网站</a></li>
          <li><a href="/sitemap.xml" target="_blank">网站地图</a></li>
        </ul>
      </div>
      <div class="qrcode">
        <ul>
          <li>
            <img src="../assets/img/qr1.png" alt="" />
            <span>官方公众号</span>
          </li>
          <li>
            <img src="../assets/img/qr2.png" alt="" />
            <span>运营小助手</span>
          </li>
        </ul>
        <p>关注公众号，不断推送最新最好用的AI工具</p>
      </div>
    </footer>
  </div>
</template>

<style lang="scss" scoped>
.footer {
  padding: 30px 48px;
  background-color: #fff;
  border-top: 1px solid #e7e7ff;

  footer {
    display: flex;
    justify-content: space-between;
    width: 1120px;
    margin: 0 auto;
  }

  .contact {
    .logo {
      display: flex;
      align-items: center;
      .icon-logo {
        display: block;
        font-size: 30px;
      }
      h1 {
        margin: 0 0 0 8px;
        font-size: 21px;
        color: #222;
        font-weight: bold;
        line-height: 1;
      }
    }
    ul {
      li {
        padding-top: 12px;
        &:last-child {
          color: #666;
          font-size: 12px;
        }
        span {
          margin-right: 8px;
        }
        a {
          color: #666;
          font-size: 12px;
          &.mailbox {
            color: #444;
            font-size: 15px;
            &:hover {
              color: #504ded;
            }
          }
        }
      }
    }
  }

  .link {
    padding-top: 12px;
    text-align: center;
    .title {
      font-size: 18px;
      font-weight: bold;
    }
    ul {
      li {
        padding-top: 12px;
        font-size: 14px;
        color: #666;
        a {
          color: #666;
          font-size: 14px;
        }
      }
    }
  }

  .qrcode {
    padding-left: 60px;
    border-left: 1px solid #e7e7ff;
    ul {
      display: flex;
      justify-content: space-between;
      margin-bottom: 10px;
      li {
        padding: 10px 10px 6px;
        text-align: center;
        // border: 1px solid #e7e7ff;
        background-color: #f8f8f8;
        border-radius: 8px;
        &:first-child {
          margin-right: 20px;
        }
        img {
          display: block;
          width: 100px;
          height: 100px;
          margin-bottom: 4px;
        }
        span {
          font-size: 12px;
        }
      }
    }
    p {
      font-size: 12px;
      text-align: center;
    }
  }
}
</style>
