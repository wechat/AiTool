
export const useMain = () => {
  const sidebar = useState("sidebar", () => [
    { icon: 'icon-hot', name: 'AI热门推荐', actived: true },
    { icon: 'icon-talk', name: 'AI对话聊天', actived: false },
    { icon: 'icon-pen', name: 'AI写作工具', actived: false },
    { icon: 'icon-image', name: 'AI图像工具', actived: false },
    { icon: 'icon-video', name: 'AI视频工具', actived: false },
    { icon: 'icon-office', name: 'AI办公工具', actived: false },
    { icon: 'icon-window-grid', name: 'AI设计工具', actived: false },
    { icon: 'icon-code', name: 'AI编程工具', actived: false },
    { icon: 'icon-record-audio', name: 'AI音频工具', actived: false },
    { icon: 'icon-translate', name: 'AI语言翻译', actived: false },
    { icon: 'icon-file-alt', name: 'AI内容检测', actived: false },
    { icon: 'icon-lightbulb-alt', name: 'AI提示指令', actived: false },
    { icon: 'icon-cube', name: 'AI训练模型', actived: false },
    { icon: 'icon-ruler-combined', name: 'AI模型评测', actived: false },
    { icon: 'icon-graduation-hat', name: 'AI学习网站', actived: false },
    { icon: 'icon-layer-group', name: 'AI开发框架', actived: false }
  ]);

  const hfSidebar = useState("hfSidebar", () => [
    { icon: 'icon-cube', name: 'Models', actived: true },
    { icon: 'icon-server-alt', name: 'Datasets', actived: false },
    { icon: 'icon-apps', name: 'Spaces', actived: false }
  ]);

  const phSidebar = useState("phSidebar", () => [
    { icon: 'icon-calendar-alt', name: 'Today', actived: true },
    { icon: 'icon-calendar-alt', name: 'Yesterday', actived: false },
    { icon: 'icon-calendar-alt', name: 'Week', actived: false },
    { icon: 'icon-calendar-alt', name: 'Month', actived: false }
  ]);

  function setActived(index: number) {
    sidebar.value.map((item, idx) => {
      if (index === idx) {
        item.actived = true
      } else {
        item.actived = false
      }
    })
  }

  function scrollTo(offsetTop: number) {
    if (!import.meta.env.SSR) {
      window.scrollTo({
        top: offsetTop - 100,
        behavior: 'smooth'
      })
    }
  }

  function anchorPosition(anchor: string) {
    if (!import.meta.env.SSR) {
      const element: HTMLAnchorElement | null = document.querySelector(anchor)
      element && scrollTo(element.offsetTop)
    }
  }

  return {
    sidebar,
    hfSidebar,
    phSidebar,
    scrollTo,
    setActived,
    anchorPosition
  }
}
