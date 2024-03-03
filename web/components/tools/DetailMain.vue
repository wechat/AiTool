<script setup lang="ts">
import { ref } from "vue";
import QRious from "qrious";
import config from "~/config";

const { sidebar } = useMain();
const { detail, categoryList, fetchCategoryListApi } = useTools();
const qrlink = ref("");

onServerPrefetch(async () => {
  if (detail.value?.category_id) {
    await fetchCategoryListApi(detail.value?.category_id, 0, 6);
  }
});

if (!import.meta.env.SSR && detail.value?.link) {
  var qr = new QRious({
    value: detail.value?.link,
  });
  qrlink.value = qr.toDataURL();
}
</script>

<template>
  <div class="main">
    <article v-if="detail">
      <div class="content">
        <div class="content-left">
          <img
            :src="`${config.CDN_URL}/image/${detail.cover}`"
            :alt="decodeURIComponent(detail.title)"
          />
          <ul>
            <li>
              <ATooltip content="点赞">
                <span class="iconfont icon-thumbs-up"></span>
                <span>{{ detail.likes }}</span>
              </ATooltip>
            </li>
            <li>
              <ATooltip content="浏览">
                <span class="iconfont icon-eye"></span>
                <span>{{ detail.reads }}</span>
              </ATooltip>
            </li>
          </ul>
        </div>
        <div class="content-right">
          <div class="breadcrumb">
            <a :href="`/category?category_id=${detail.category_id}`">{{
              sidebar[detail.category_id].name
            }}</a>
            <ul>
              <li>&gt;</li>
              <li v-for="(item, index) in detail.tags" :key="index">
                <a :href="`/tag?tag=${encodeURIComponent(item)}`">{{ item }}</a>
              </li>
            </ul>
          </div>
          <h1>{{ decodeURIComponent(detail.title) }}</h1>
          <p>{{ decodeURIComponent(detail.describe) }}</p>
          <ul class="tag">
            <li>标签:</li>
            <li v-for="(item, index) in detail.tags" :key="index">
              <a :href="`/tag?tag=${encodeURIComponent(item)}`"
                >{{ item }}<span class="iconfont icon-open-link"></span
              ></a>
            </li>
          </ul>
          <ul class="btn">
            <li>
              <AHref class="link" :link="detail.link">
                <span>访问官网</span>
                <span class="iconfont icon-angle-right-b"></span>
              </AHref>
            </li>
            <li>
              <APopover>
                <a href="javascript:;"
                  >手机查看<span class="iconfont icon-scan"></span
                ></a>
                <template #content>
                  <img :src="qrlink" alt="" />
                </template>
              </APopover>
            </li>
            <li>
              <ATooltip content="反馈">
                <a :href="`mailto:${config.MAILBOX}`" target="_blank"
                  ><span class="iconfont icon-feedback"></span
                ></a>
              </ATooltip>
            </li>
          </ul>
        </div>
      </div>
      <main>
        <div class="main-left">
          <div
            class="main-left-content"
            v-if="detail.content"
            v-html="decodeURIComponent(detail.content)"
          ></div>
          <div class="title">类似工具</div>
          <ul class="similar">
            <li v-for="(item, index) in categoryList" :key="index">
              <a :href="item.link" target="_blank">
                <div class="similar-left">
                  <img
                    :src="`${config.CDN_URL}/icon/${item.icon}`"
                    :alt="decodeURIComponent(item.title)"
                  />
                </div>
                <div class="similar-right">
                  <h3>{{ decodeURIComponent(item.title) }}</h3>
                  <p>{{ decodeURIComponent(item.describe) }}</p>
                </div>
              </a>
            </li>
          </ul>
          <div class="title">暂无评价</div>
          <div class="comment">
            <form>
              <textarea
                name=""
                cols="30"
                rows="5"
                placeholder="即将上线，敬请期待"
                disabled
              ></textarea>
              <div class="btn">
                <ul>
                  <li>
                    昵称<input type="text" placeholder="请输入昵称" disabled />
                  </li>
                  <li>
                    邮箱<input
                      type="text"
                      placeholder="请输入联系邮箱"
                      disabled
                    />
                  </li>
                </ul>
                <button disabled>发表评价</button>
              </div>
            </form>
          </div>
        </div>
        <div class="main-right">
          <AHref class="block" link="https://curl.qcloud.com/oCjBPIYs">
            <img
              src="../../assets/img/txc.png"
              :alt="decodeURIComponent(detail.title)"
            />
          </AHref>
        </div>
      </main>
    </article>
  </div>
</template>

<style lang="scss" scoped>
.main {
  flex: 1;
  article {
    width: 1120px;
    margin: 0 auto;
    h1 {
      margin: 0;
    }
  }
  .content {
    display: flex;
    padding: 50px 0 80px;
    &-left {
      flex-shrink: 0;
      overflow: hidden;
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      width: 260px;
      height: 260px;
      padding: 10px;
      background: #e6e8ed;
      border-radius: 10px;
      transition: background-color 0.3s;
      box-sizing: border-box;
      box-shadow: 0 30px 20px -20px rgba(0, 0, 0, 0.15);
      img {
        max-width: 100%;
        max-height: 100%;
      }
      ul {
        position: absolute;
        right: 0;
        left: 0;
        bottom: 10px;
        z-index: 2;
        display: flex;
        justify-content: center;
        li {
          width: 58px;
          height: 58px;
          margin: 0 8px;
          cursor: pointer;
          text-align: center;
          border-radius: 50%;
          background-color: #eff1f5;
          span {
            display: block;
            font-size: 12px;
            line-height: 1;
          }
          .iconfont {
            margin-bottom: 4px;
            padding-top: 8px;
            font-size: 24px;
          }
          &:hover {
            background-color: #fff;
          }
        }
      }
    }
    &-right {
      overflow: hidden;
      flex: 1;
      margin-left: 20px;
      padding-top: 10px;
      .breadcrumb {
        display: flex;
        align-items: center;
        a {
          padding: 2px 5px;
          font-size: 12px;
          color: #fff;
          background-color: #5961f9;
          border-color: #5961f9;
          border-radius: 4px;
          transition: 0.3s;
        }
        ul {
          display: flex;
          margin-left: 6px;
          li {
            margin-right: 6px;
            color: #666;
            font-size: 12px;
          }
        }
      }
      h1 {
        padding: 16px 0 20px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
      p {
        overflow: hidden;
        margin-bottom: 15px;
        color: #333;
        line-height: 1.5;
        text-overflow: ellipsis;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
      }
      .tag {
        display: flex;
        margin-bottom: 24px;
        li {
          margin-right: 12px;
          .iconfont {
            font-size: 10px;
          }
          a:hover {
            text-decoration: underline;
          }
        }
      }
      .btn {
        display: flex;
        li {
          margin-right: 15px;
          a,
          .link {
            display: block;
            padding: 12px 16px;
            color: #000;
            border-radius: 8px;
            background-color: #e9e9e9;
            cursor: pointer;
            &:hover {
              color: #fff;
              background-color: #504ded;
            }
            span {
              margin-left: 6px;
            }
          }
          &:last-child {
            a,
            .link {
              display: flex;
              align-items: center;
              justify-content: center;
              color: #fff;
              background-color: #de5e44;
              span {
                margin-left: 0;
                font-size: 22px;
                line-height: 1;
              }
            }
          }
        }
      }
    }
  }
  main {
    display: flex;
    justify-content: space-between;
    .main-left {
      flex: 1;
      padding: 30px 18px 30px;
      border-radius: 8px;
      background-color: #fff;
      .title {
        position: relative;
        padding: 10px 0 15px 18px;
        color: #333;
        font-size: 22px;
        font-weight: bold;
        &::after {
          content: "";
          position: absolute;
          top: 15px;
          left: 0;
          bottom: 4px;
          width: 8px;
          height: 20px;
          border-radius: 2px;
          background-color: #504ded;
        }
      }

      &-content {
        margin-bottom: 30px;
        font-size: 16px !important;
        line-height: 1.6 !important;
      }
      .similar {
        display: flex;
        flex-wrap: wrap;
        margin-bottom: 10px;
        li {
          width: 33%;
          flex-shrink: 0;
          flex-wrap: wrap;
          margin-bottom: 15px;

          &:nth-child(3n) {
            a {
              margin-right: 0;
            }
          }
        }
        a {
          display: flex;
          align-items: center;
          padding: 10px;
          height: 100px;
          margin-right: 15px;
          border-radius: 8px;
          background-color: #f9f9f9;
          box-sizing: border-box;
          transition: background-color 0.3s;

          img {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            margin-right: 12px;
          }
          h3 {
            margin: 0 0 5px;
            font-size: 16px;
            color: #333;
            font-weight: bold;
          }
          p {
            margin: 0;
            font-size: 12px;
            color: #999;
            overflow: hidden;
            text-overflow: ellipsis;
            display: -webkit-box;
            -webkit-line-clamp: 2;
            -webkit-box-orient: vertical;
          }
        }
      }

      .comment {
        textarea {
          width: 100%;
          margin-bottom: 10px;
          padding: 20px;
          border: none;
          resize: none;
          border-radius: 8px;
          box-sizing: border-box;
          background-color: #f9f9f9;
        }
        .btn {
          display: flex;
          align-items: center;
          justify-content: space-between;
          li {
            margin-right: 20px;
          }
          button {
            padding: 14px 20px;
            color: #fff;
            border-radius: 8px;
            background-color: #e9e9e9;
            &.active {
              background-color: #504ded;
            }
          }
        }
        ul {
          display: flex;
          align-items: center;
          li {
            input {
              margin-left: 10px;
              padding: 0 10px;
              width: 200px;
              height: 48px;
              border-radius: 8px;
              background-color: #f9f9f9;
            }
          }
        }
      }
    }
    .main-right {
      flex-shrink: 0;
      width: 300px;
      margin-left: 20px;
      border-radius: 8px;
      img {
        width: 100%;
      }
      .block {
        overflow: hidden;
        margin-bottom: 10px;
        border-radius: 8px;
      }
    }
  }
}
</style>
<style lang="scss">
.main-left-content {
  h1 {
    margin: 0 0 20px;
    font-size: 24px;
    font-weight: bold;
  }
  h2 {
    margin: 0 0 15px;
    font-size: 20px;
    font-weight: bold;
  }
  h3 {
    margin: 0 0 15px;
    font-size: 18px;
    font-weight: bold;
  }
  blockquote {
    margin-bottom: 20px;
    padding: 24px;
    background-color: #f5f5f5;
    border-radius: 8px;
    p:last-child {
      margin-bottom: 0;
    }
  }
  p {
    margin: 0 0 20px;
    font-size: 15px;
    line-height: 1.6;
    color: #333;
  }
  ul {
    margin-bottom: 30px;
    li {
      margin-bottom: 10px;
    }
  }
  ol {
    list-style: circle;
    margin-bottom: 30px;
    li {
      margin-bottom: 10px;
    }
  }
  table {
    width: 100%;
    margin-bottom: 30px;
    border-collapse: collapse;
    th {
      padding: 12px 10px;
      border: 1px solid #e5e5e5;
    }
    td {
      padding: 10px;
      border: 1px solid #e5e5e5;
    }
  }
}
</style>
