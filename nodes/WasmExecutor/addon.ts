// The Rust addon.
import * as addon from './load.cjs';

export interface WasiExecution {
  args: Array<string>,
  program: string,
  stdin: string,
  env: {[key: string]: string}
}

export interface WasiResult {
  failed: boolean,
  stdout: string,
  stderr: string,
  status_code: number
}

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function execute_wasm(exec: WasiExecution): WasiResult;
}

export function execute_wasm(exec: WasiExecution): WasiResult {
  const result = addon.execute_wasm(exec);
  return result;
}

export function execute_wasm_example(): WasiResult {
  return execute_wasm({
    program: "target/wasm32-wasip2/debug/wasi.wasm",
    args: [],
    env: {
      "APP_NAME": "Hello"
    },
    stdin: "Line1\nLine2\nLine3"
  });
}
