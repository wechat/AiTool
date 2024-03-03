<script setup lang="ts">
import config from "~/config";
</script>

<template>
  <div class="header">
    <header>
      <div class="logo">
        <a href="/"
          ><span class="iconfont icon-logo"></span>{{ config.DOMAIN }}</a
        >
      </div>
      <div class="head">
        <nav>
          <ul class="nav">
            <li class="on"><a href="/">首页</a></li>
            <li><a href="/new">最新推出</a></li>
            <li><a href="/hot">热门推荐</a></li>
            <!-- <li><a href="/often">常用工具</a></li>
            <li><a href="/often">出海工具</a></li> -->
            <li>
              <a :href="config.SUBMIT_URL" target="_blank">提交网站</a>
            </li>
          </ul>
          <ul class="user">
            <li>
              <a href="/search" class="search"
                ><span class="iconfont icon-search"></span
              ></a>
            </li>
            <li><a href="/login">登录</a></li>
            <li><a href="/signup">注册</a></li>
          </ul>
        </nav>
      </div>
    </header>
  </div>
</template>

<style lang="scss" scoped>
.header {
  height: 80px;
  header {
    position: fixed;
    top: 0;
    right: 0;
    left: 0;
    z-index: 999;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 0 48px;
    height: 80px;
    font-size: 16px;
    font-weight: 700;
    line-height: 1;
    border-bottom: 1px solid #e7e7ff;
    backdrop-filter: blur(8px);
    background: rgba(255, 255, 255, 0.8);

    .logo {
      width: 204px;
      a {
        display: flex;
        align-items: center;
        width: 168px;
        color: #000;
        font-size: 19px;
        font-weight: bold;
      }
      img {
        display: block;
        width: 100%;
      }
      h1 {
        font-size: 19px;
        text {
          font-size: 20px;
        }
      }
      .icon-logo {
        margin-right: 8px;
        color: #413fe6;
        font-size: 38px;
      }
    }

    .head {
      flex: 1;
    }

    nav {
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .nav {
      display: flex;
      align-items: center;

      li {
        border-radius: 8px;
        &:first-child {
          a {
            min-width: 90px;
            box-sizing: border-box;
          }
        }
        a {
          display: flex;
          align-items: center;
          justify-content: center;
          // min-width: 94px;
          padding: 0 18px;
          height: 50px;
          color: #222222;
          font-size: 15px;
          font-weight: bold;
          border-radius: 8px;
          &:hover {
            background-color: #f5f6fd;
          }
        }
        &.on {
          background-color: #f5f6fd;
        }
        .icon {
          margin: -2px 6px 0 0;
          width: 20px;
          height: 20px;
        }
        .icon-product-hunt {
          margin: -2px 4px 0 0;
          color: #ff6154;
          font-size: 22px;
        }
      }
    }

    .user {
      display: flex;
      justify-items: center;
      justify-content: flex-end;

      li {
        a {
          display: block;
          padding: 14px;
          border-radius: 8px;
          box-sizing: border-box;
        }

        &:last-child {
          a {
            margin-left: 16px;
            padding: 14px 20px;
            color: #fff;
            background-color: #504ded;
          }
        }

        .search {
          padding: 14px 12px 12px;
          color: #222222;
          &:hover {
            background-color: #f5f6fd;
          }
        }
      }
    }
  }
}
</style>
