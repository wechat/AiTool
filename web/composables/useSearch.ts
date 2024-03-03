import { type IToolsListItem } from "~/types/tools";
import { useRequest } from "~/composables/useRequest";

export const useSearch = () => {
  const searchList = useState("searchList", (): IToolsListItem[] => []);

  async function fetchSearchListApi(
    keyword: string,
    pageNum: number = 0,
    pageSize: number = 500
  ) {
    let result = []
    result = await useRequest<IToolsListItem[]>(`/api/search/list`, {
      params: {
        keyword,
        page_num: pageNum,
        page_size: pageSize,
      },
    });
    if (result) {
      searchList.value = result;
    }
    return result;
  }

  return {
    searchList,
    fetchSearchListApi,
  };
};
