import {
  type IToolsListItem,
  type IToolsDetailItem,
  type IToolsHomeList,
} from "~/types/tools";
import { useRequest } from "~/composables/useRequest";

export const useTools = () => {
  // 热门推荐
  const hotList = useState("hotList", (): IToolsListItem[] => []);

  // 热门推荐
  const newList = useState("newList", (): IToolsListItem[] => []);

  // 热门推荐
  const categoryList = useState("categoryList", (): IToolsListItem[] => []);

  // 标签列表
  const tagsList = useState("tagsList", (): IToolsListItem[] => []);

  // 工具首页
  const homeList = useState("homeList", (): IToolsHomeList => []);

  // 工具详情
  const detail = useState("detail", (): IToolsDetailItem | null => null);

  async function fetchHotListApi(pageNum: number = 0, pageSize: number = 18) {
    let result = []
    result = await useRequest<IToolsListItem[]>("/api/tools/hot", {
      params: {
        page_num: pageNum,
        page_size: pageSize,
      },
    });
    if (result) {
      hotList.value = result;
    }
    return result;
  }

  async function fetchNewListApi(pageNum: number = 0, pageSize: number = 18) {
    let result = []
    result = await useRequest<IToolsListItem[]>("/api/tools/new", {
      params: {
        page_num: pageNum,
        page_size: pageSize,
      },
    });
    if (result) {
      newList.value = result;
    }
    return result;
  }

  async function fetchHomeListApi(pageNum: number = 0, pageSize: number = 18) {
    const result = await useRequest<IToolsHomeList>("/api/tools/list", {
      params: {
        page_size: pageSize,
      },
    });
    homeList.value = result;
    return result;
  }

  async function fetchToolDetailApi(tid: string) {
    const result = await useRequest<IToolsDetailItem>(
      `/api/tools/detail/${tid}`
    );
    detail.value = result;
    return result;
  }

  async function fetchTagsListApi(
    tag: string,
    pageNum: number = 0,
    pageSize: number = 18
  ) {
    let result = []
    result = await useRequest<IToolsListItem[]>(`/api/tools/tags`, {
      params: {
        tag,
        page_num: pageNum,
        page_size: pageSize,
      },
    });
    if (result) {
      tagsList.value = result;
    }
    return result;
  }

  async function fetchCategoryListApi(
    category_id: number,
    pageNum: number = 0,
    pageSize: number = 18
  ) {
    let result = []
    result = await useRequest<IToolsListItem[]>(`/api/tools/category`, {
      params: {
        category_id,
        page_num: pageNum,
        page_size: pageSize,
      },
    });
    if (result) {
      categoryList.value = result;
    }
    return result;
  }

  return {
    hotList,
    newList,
    homeList,
    detail,
    tagsList,
    categoryList,
    fetchHotListApi,
    fetchNewListApi,
    fetchHomeListApi,
    fetchToolDetailApi,
    fetchTagsListApi,
    fetchCategoryListApi,
  };
};
