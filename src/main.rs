mod client_pool;
mod client_pool_ops;

use deno_core::error::AnyError;
use deno_core::{op2, PollEventLoopOptions};
use std::rc::Rc;

#[op2(fast)]
pub fn op_print_foobar() -> Result<(), AnyError> {
    println!("foobar");
    Ok(())
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let path = std::env::current_dir().unwrap();
    let main_module = deno_core::resolve_path(file_path, path.as_path())?;
    deno_core::extension!(
        my_extension,
        ops = [
            op_print_foobar,
            client_pool_ops::op_client_pool_connect,
            client_pool_ops::op_client_pool_new,
            client_pool_ops::op_client_pool_publish
        ]
    );
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![my_extension::init_ops()],
        ..Default::default()
    });
    js_runtime
        .execute_script_static("[customjs:custom.js]", include_str!("./js/runtime.js"))
        .unwrap();
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;
    result.await
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js("example.js")) {
        eprintln!("error: {}", error);
    }
}
