// @ts-ignore Wasmify and ComponentizeJS will resolve this
// import but your editor may not be aware
import { log } from "wasi:logging/logging";

export function subtract(a: number, b: number): number {
  const result = a - b;

  log("info", "guest:javascript:subtract", `${a} - ${b} = ${result}`);

  return result;
}
