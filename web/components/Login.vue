<script setup lang="ts">
import config from '~/config';

interface Props {
  type: string;
}
defineProps<Props>();
</script>

<template>
  <div class="login">
    <div class="logo">
      <a href="/"><span class="iconfont icon-logo"></span>{{ config.DOMAIN }}</a>
    </div>
    <div class="form">
      <div class="username">
        <input type="text" placeholder="请输入用户名/邮箱" />
      </div>
      <div class="password">
        <input type="password" placeholder="请输入密码" />
      </div>
      <button class="submit" v-if="type === 'login'">登录</button>
      <button class="submit" v-else>注册</button>
    </div>
    <a href="/" class="forgot-pass" v-if="type === 'login'">忘记密码?</a>
    <div v-else class="login-tip">
      <span>已有账号，</span><a href="/login" class="forgot-pass">马上登录</a>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.login {
  width: 400px;
  padding: 30px;
  text-align: center;
  border-radius: 8px;
  background-color: #fff;

  .logo {
    margin-bottom: 15px;
    text-align: center;
    a {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 168px;
      margin: 0 auto;
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

  .username {
    border-bottom: 1px solid #e5e5e5;
    input {
      width: 100%;
      height: 58px;
    }
  }

  .password {
    margin-bottom: 40px;
    border-bottom: 1px solid #e5e5e5;
    input {
      width: 100%;
      height: 58px;
    }
  }

  .submit {
    width: 100%;
    margin-bottom: 30px;
    padding: 20px 0;
    font-size: 16px;
    color: #a0a0a0;
    border-radius: 8px;
    background-color: #e5e5e5;
  }

  .forgot-pass {
    color: #a0a0a0;
    font-size: 14px;
  }

  .login-tip {
    color: #a0a0a0;
    font-size: 14px;
    a {
      color: #413fe6;
    }
  }
}
</style>
