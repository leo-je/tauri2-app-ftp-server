import { ref, nextTick } from 'vue';

export function useFirstLoadAnimation(delay = 600) {
  const isFirstLoad = ref(true);

  nextTick(() => {
    setTimeout(() => {
      isFirstLoad.value = false;
    }, delay);
  });

  return { isFirstLoad };
}
