declare interface Window {
  clarity: any
  dataLayer: any
  __INITIAL_STATE__: string
}

declare module 'qrious'

interface IObject<T> {
  [key: string]: T
}
