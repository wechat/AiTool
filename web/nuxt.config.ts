// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: false },
  ssr: true,
  app: {
    head: {
      title: "发现最好的AI工具 AiTool.Run",
      htmlAttrs: {
        lang: "zh-CN",
      },
      meta: [
        {
          charset: "utf-8",
        },
        {
          name: "viewport",
          content: "width=device-width, initial-scale=1",
        },
        {
          name: "keywords",
          content:
            "AiTool,AI人工智能,Ai Tools 网站,AI学习,AI方向,AI趋势,AI工具,AI大模型,AI写作,AI绘画,AI问答,AI工具集,AI工具排行,AI导航,AI工具集网站,AI工具集导航,AI工具集导航官网,AI工具导航,AI工具箱,AI工具箱导航,AI工具大全,AI网站大全,AI软件大全,AI工具集合,AI工具集合网,AI工具库,Ai Tool,Ai Tools,AI写作免费",
        },
        {
          name: "description",
          content:
            "AiTool收录了国内外数百个AI人工智能工具，涵盖了AI对话、AI写作、AI图像生成和背景移除、AI视频制作、AI音频转录、AI辅助编程、AI音乐生成、AI绘画设计、AI对话聊天等AI工具集合大全，以及AI学习开发的常用网站、框架和模型，帮助你加入AI人工智能浪潮，智能自动化提高你的效率！",
        },
        {
          name: "baidu-site-verification",
          content: "codeva-GhGbZF4lto",
        },
      ],
      link: [
        {
          rel: "icon",
          type: "image/svg+xml",
          href: "/icon.svg",
        },
      ],
      script: [
        {
          src: "https://www.googletagmanager.com/gtag/js?id=G-TCF2KZ7FFK",
          async: true,
        },
        { src: "/js/analytics.js" },
        { src: "/js/clarity.js" },
        {
          src: "https://www.clarity.ms/tag/kg40zicahq",
          async: true,
        },
      ],
    },
    keepalive: true,
  },
  css: ["~/assets/css/style.css", "~/assets/css/iconfont.css"],
  modules: ["@nuxtjs/i18n", "arco-design-nuxt-module"],
  arco: {
    importPrefix: "A",
    hookPrefix: "Arco",
    locales: ["getLocale"],
    localePrefix: "Arco",
  },
  nitro: {
    storage: {
      redis: {
        driver: "redis",
        port: 6379,
        host: "127.0.0.1",
        username: "",
        password: "",
        db: 0,
        tls: {},
      },
    },
  },
});
