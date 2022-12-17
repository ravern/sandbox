import { DependencyList, useCallback } from "react"

export default function useAsyncCallback<T extends (...args: any[]) => Promise<any>>(callback: T, deps: DependencyList): T {
  return useCallback(async (...args: Parameters<T>) => {
    try {
      return callback(...args);
    } catch (error) {
      console.error(error);
    }
  }, deps) as T;
}