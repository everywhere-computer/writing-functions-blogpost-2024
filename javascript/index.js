import { componentize } from "@bytecodealliance/componentize-js";
import { readFileSync } from "node:fs";
import { writeFile } from "node:fs/promises";

const jsSource = readFileSync("src/subtract.js").toString();
const { component } = await componentize(jsSource, {
  witPath: "../wit",
  worldName: "subtraction",
});

await writeFile("output/subtract.wasm", component);
