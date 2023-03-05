export function timeoutValue<T>(returnValue: T, timeout: number): Promise<T> {
  return new Promise((resolve) =>
    setTimeout(() => resolve(returnValue), timeout)
  );
}
