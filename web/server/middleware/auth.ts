export default defineEventHandler((event) => {
  event.context.auth = { user: 'auth' }
})
