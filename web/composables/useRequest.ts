import { type UseFetchOptions } from "nuxt/app";
import config from '~/config';

export interface IResponse<T> {
  code: number;
  data: T;
  msg: string;
}

export const useRequest = async <T = unknown>(
  url: string,
  opts?: UseFetchOptions<T>
) => {
  const defaultOptions: UseFetchOptions<T> = {
    baseURL: config.API_BASE_URL,
    // 设置请求头
    onRequest({ options }) {},
    // 处理响应数据
    onResponse({ response }) {},
    // 处理响应错误
    onResponseError({ response }) {},
  };
  const { data } = await useFetch(url, { ...defaultOptions, ...opts });
  const result: IResponse<T> = data.value as IResponse<T>;
  return result?.data;
};
