import { ref } from 'vue';

export function useRunTimer() {
  const runTime = ref('00:00:00');
  const startTime = ref<Date | null>(null);
  let runTimer: ReturnType<typeof setInterval> | null = null;

  const formatRunTime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const start = () => {
    startTime.value = new Date();
    runTimer = setInterval(() => {
      if (startTime.value) {
        const elapsed = Math.floor((Date.now() - startTime.value.getTime()) / 1000);
        runTime.value = formatRunTime(elapsed);
      }
    }, 1000);
  };

  const stop = () => {
    if (runTimer) {
      clearInterval(runTimer);
      runTimer = null;
    }
    startTime.value = null;
    runTime.value = '00:00:00';
  };

  return { runTime, startTime, start, stop };
}
