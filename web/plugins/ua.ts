export default defineNuxtPlugin(() => {
  return {
    provide: {
      isMobile: () => {
        const { origin } = useRequestURL()
        return origin.includes('mobile.')
      },
    },
  };
});
