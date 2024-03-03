export interface IToolsListItem {
  tid: string
  icon: string
  title: string
  describe: string
  link: string
  category_id?: number
  sub_category_id?: number
}

export interface IToolsDetailItem {
  tid: string
  cover: string
  title: string
  describe: string
  link: string
  category_id: number
  sub_category_id: number
  tags: string[]
  likes: number
  reads: number
  collects: number
  content: string
  is_hot: boolean
  is_new: boolean
}

export interface IToolsHomeList {
  [key: number]: IToolsListItem[]
}
