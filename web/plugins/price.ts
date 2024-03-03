export default defineNuxtPlugin(() => {
  return {
    provide: {
      formPrice: (price: number) => {
        return price.toFixed(2);
      },
    },
  };
});
