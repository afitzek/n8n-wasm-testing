use interface::{WasiError, WasiExecution, WasiResult};
use neon::prelude::*;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::{
    bindings::sync::Command,
    pipe::{MemoryInputPipe, MemoryOutputPipe},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

mod interface;

struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::add_to_linker_sync`]
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
		http: WasiHttpCtx,
    // You can add other custom host states if needed
}

impl IoView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}
impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl WasiHttpView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}

fn exec(exec_arguments: WasiExecution) -> wasmtime::Result<WasiResult> {
    let mut config = Config::new();
    config.async_support(false);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
		wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;


		println!("Executing WASI program1 {}", exec_arguments.program);

    let mut wasi = WasiCtxBuilder::new();

    for (k, v) in exec_arguments.env_map.iter() {
        wasi.env(k, v);
    }

    for arg in exec_arguments.args.iter() {
        wasi.arg(arg);
    }

    let stdin = exec_arguments.stdin.to_owned();
    let stdin = MemoryInputPipe::new(stdin);

    wasi.stdin(stdin);

    let stdout = MemoryOutputPipe::new(4096);
    wasi.stdout(stdout.clone());
    let stderr = MemoryOutputPipe::new(4096);
    wasi.stderr(stderr.clone());

    let wasi = wasi.build();

    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
        http: WasiHttpCtx::new(),
    };
    let mut store = Store::new(&engine, state);

    // Instantiate our component with the imports we've created, and run it.
    let component = Component::from_file(&engine, exec_arguments.program)?;
    let command = Command::instantiate(&mut store, &component, &linker)?;
    let program_result = command.wasi_cli_run().call_run(&mut store)?;

    let result = WasiResult {
        failed: program_result.is_err(),
        stdout: String::from_utf8_lossy(stdout.contents().as_ref()).to_string(),
        stderr: String::from_utf8_lossy(stderr.contents().as_ref()).to_string(),
        status_code: 0,
    };

    Ok(result)
}

fn execute_wasm(mut cx: FunctionContext) -> JsResult<JsObject> {
    let exec_req = WasiExecution::from_ctx(&mut cx);

    if let Ok(exec_req) = exec_req {
        let result = exec(exec_req);
        return match result {
            Ok(r) => {
                r.to_js_object(&mut cx)
            },
            Err(e) => {
                WasiError{
                    error: e.to_string()
                }.to_js_object(&mut cx)
            }
        }
    }
    WasiError{
        error: "Failed to parse wasi execution request".to_string()
    }.to_js_object(&mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("execute_wasm", execute_wasm)?;

    Ok(())
}
