mod ast_replacer;
mod plugins;
mod tab;

use std::sync::mpsc::channel;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::Duration;

use ast_replacer::lib::AstReplacer;
use ast_replacer::utils::transform_to_result;
use oxc_allocator::Allocator;

use oxc_ast::AstBuilder;
use oxc_codegen::{CodeGenerator, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use rustyscript::deno_core::error::AnyError;
use rustyscript::RuntimeOptions;
use tab::Tab;
use tauri::Manager;

use rustyscript::Error;
use rustyscript::Runtime;
use serde_json::Value;
use tauri_plugin_svelte::ManagerExt;

const STORE_NAME: &str = "storage2";
const TABS_KEY: &str = "tabs";

#[tauri::command]
async fn handle_editor_changes(
    source_text: String,
    tab_id: String,
    app: tauri::AppHandle,
) -> Result<(), Error> {

    // Retrieve current tabs.
    let mut tabs_data = app
        .svelte()
        .try_get::<Vec<Tab>>(STORE_NAME, TABS_KEY)
        .unwrap();

    // Create a channel to receive the thread-safe handle.
    let (tx_handle, rx_handle) = channel();
    // Create a channel to receive the worker result.
    let (tx_result, rx_result) = channel();

    // Clone the source text for the worker thread.
    let source_text_worker = source_text.clone();

    // Spawn a dedicated worker thread.
    let join_handle = thread::spawn(move || -> Result<(String, Vec<String>), Error> {
        // Create a runtime for evaluating the script.
        let mut runtime = Runtime::new(RuntimeOptions {
            timeout: Duration::from_secs(2),
            ..Default::default()
        }).unwrap();

        // Immediately send the thread-safe handle so the main thread can cancel if needed.
        let ts_handle = runtime
            .deno_runtime()
            .v8_isolate()
            .thread_safe_handle();
        tx_handle.send(ts_handle).expect("Failed to send thread-safe handle");

        // Evaluate the main source text.
        let mut error_messages = Vec::new();
        if let Err(err) = runtime.eval::<()>(source_text_worker.clone()) {
            error_messages.push(err.as_highlighted(Default::default()));
        }

        // Perform parsing, semantic analysis, and AST processing.
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let ret = Parser::new(&allocator, &source_text_worker, source_type).parse();

        for error in ret.errors {
            let error = error.with_source_code(source_text_worker.clone());
            error_messages.push(format!("{:?}", error));
        }
        let _semantic_ret = SemanticBuilder::new().build(&ret.program);
        let _ast_builder = AstBuilder::new(&allocator);

        // If no errors so far, transform the code.
        let transformed_code = if error_messages.is_empty() {
            let program = allocator.alloc(ret.program);
            let _ast_pass = AstReplacer::new(&allocator, source_text_worker.clone()).build(program);
            let new_code = CodeGenerator::new()
                .with_options(CodegenOptions::default())
                .build(&program)
                .code;

            // Create a second runtime to evaluate a helper module.
            let mut runtime2 = Runtime::new(Default::default()).unwrap();
            runtime2
                .eval::<()>(
                    r#"
                    // Xtal is a function that wraps call expressions
                    globalThis.Xtal = async (line, ...valuePromise) => {
                        if (typeof globalThis.XtalResults === 'undefined') {
                            globalThis.XtalResults = [];
                        }
                        const resolvedValues = await Promise.all(
                            valuePromise.map(async (value) => await value)
                        );
                        resolvedValues.forEach(value => {
                            globalThis.XtalResults.push({ line, value });
                        });
                    };
                    "#,
                )
                .unwrap();

            if let Err(err) = runtime2.eval::<()>(new_code) {
                error_messages.push(err.as_highlighted(Default::default()));
            }

            let debug_results: Vec<Value> = runtime2
                .eval("globalThis.XtalResults")
                .unwrap_or_else(|_| Vec::new());

            transform_to_result(debug_results)
        } else {
            String::new()
        };

        let result = (transformed_code, error_messages);
        // Send the result back to the main thread.
        tx_result.send(result.clone()).expect("Failed to send result");
        Ok(result)
    });

    // In the main thread, receive the thread-safe handle.
    let ts_handle = rx_handle.recv().expect("Failed to receive thread-safe handle");

    // Wait up to 2 seconds for the worker to complete.
    let worker_result = rx_result.recv_timeout(Duration::from_secs(2));
    let (transformed_result, error_messages) = match worker_result {
        Ok(res) => res, // Worker finished quickly.
        Err(RecvTimeoutError::Timeout) => {
            // Timeout expired: terminate the execution.
            ts_handle.terminate_execution();
            // Wait for the worker thread to finish.
            join_handle.join().expect("Worker thread panicked")?
        }
        Err(e) => return Err(AnyError::msg(format!("Worker channel error: {:?}", e)).into()),
    };

    // Update the appropriate tab.
    if let Some(tab) = tabs_data.iter_mut().find(|tab| tab.id == tab_id) {
        tab.result = transformed_result;
        tab.errors = error_messages.join("\n");
    }
    let tabs_json = serde_json::to_value(&tabs_data).unwrap();
    let _ = app.svelte().set(STORE_NAME, TABS_KEY, tabs_json);

    Ok(())
}


#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    app.get_webview_window("main").unwrap().show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(plugins::tauri_traffic_light_positioner_plugin::init())
        .plugin(tauri_plugin_svelte::init())
        .invoke_handler(tauri::generate_handler![handle_editor_changes, show_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
