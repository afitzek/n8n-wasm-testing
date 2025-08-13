use std::collections::HashMap;

use neon::prelude::*;

pub(crate) struct WasiExecution {
    pub program: String,
    pub env_map: HashMap<String, String>,
    pub args: Vec<String>,
    pub stdin: String,
}

impl WasiExecution {
    pub fn from_ctx(ctx: &mut FunctionContext<'_>) -> Result<WasiExecution, String> {
        let obj: Handle<JsObject> = ctx
            .argument(0)
            .map_err(|e| format!("Failed to get argument: {}", e))?;

        let program: Handle<JsString> = obj
            .get(ctx, "program")
            .map_err(|e| format!("Failed to get `program`: {}", e))?;
        let stdin: Handle<JsString> = obj
            .get(ctx, "stdin")
            .map_err(|e| format!("Failed to get `stdin`: {}", e))?;

        let args: Handle<JsArray> = obj
            .get(ctx, "args")
            .map_err(|e| format!("Failed to get `args`: {}", e))?;
        let args_vec: Vec<Handle<JsValue>> = args
            .to_vec(ctx)
            .map_err(|e| format!("Failed to cast to vec: {}", e))?;
        let mut args: Vec<String> = Vec::with_capacity(args_vec.len());

        for arg in args_vec {
            let arg: Handle<JsString> = arg
                .downcast(ctx)
                .map_err(|e| format!("Failed to cast arg to string: {}", e))?;
            args.push(arg.value(ctx));
        }

        let env_map_js: Handle<JsObject> = obj
            .get(ctx, "env")
            .map_err(|e| format!("Failed to get `env`: {}", e))?;
        let properties = env_map_js
            .get_own_property_names(ctx)
            .map_err(|e| format!("Failed to get `get_own_property_names`: {}", e))?
            .to_vec(ctx)
            .map_err(|e| format!("Failed to convert own props to vector: {}", e))?;

        let properties_len = properties.len();

        let mut env_map: HashMap<String, String> = HashMap::with_capacity(properties_len);

        for property in properties {
            let property_name: Handle<JsString> = property
                .downcast(ctx)
                .map_err(|e| format!("Failed to cast property to string: {}", e))?;
            let property_name = property_name.value(ctx);
            let value: Handle<JsValue> = env_map_js
                .get(ctx, property_name.as_str())
                .map_err(|e| format!("Failed to get property value: {}", e))?;
            let value: Handle<JsString> = value
                .downcast(ctx)
                .map_err(|e| format!("Failed to cast property value to string: {}", e))?;
            env_map.insert(property_name, value.value(ctx));
        }

        Ok(WasiExecution {
            program: program.value(ctx),
            env_map,
            args,
            stdin: stdin.value(ctx),
        })
    }
}

pub(crate) struct WasiError {
    pub error: String,
}

impl<'a> WasiError {
    pub fn to_js_object(&self, ctx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = ctx.empty_object();
        let error = ctx.string(&self.error);
        obj.set(ctx, "error", error)?;
        Ok(obj)
    }
}

pub(crate) struct WasiResult {
    pub failed: bool,
    pub stdout: String,
    pub stderr: String,
    pub status_code: i32,
}

impl<'a> WasiResult {
    pub fn to_js_object(&self, ctx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = ctx.empty_object();

        let failed = ctx.boolean(self.failed);
        obj.set(ctx, "failed", failed)?;

        let stdout = ctx.string(&self.stdout);
        obj.set(ctx, "stdout", stdout)?;

        let stderr = ctx.string(&self.stderr);
        obj.set(ctx, "stderr", stderr)?;

        let year = ctx.number(self.status_code);
        obj.set(ctx, "status_code", year)?;

        Ok(obj)
    }
}
