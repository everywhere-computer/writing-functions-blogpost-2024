// @ts-ignore ComponentizeJS will resolve this import
// but your editor may not be aware
import { log } from "wasi:logging/logging";

export function subtract(a, b) {
  const result = a - b;

  log("info", "guest:js:subtract", `${a} - ${b} = ${result}`);

  return result;
}
