mod ast_replacer;
mod plugins;
mod tab;

use std::sync::Arc;
use std::time::Duration;

use ast_replacer::ast_replacer::AstReplacer;
use ast_replacer::utils::transform_to_result;
use oxc_allocator::Allocator;

use oxc_ast::AstBuilder;
use oxc_codegen::{CodeGenerator, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use rustyscript::RuntimeOptions;
use tab::Tab;
use tokio::sync::Mutex;

use rustyscript::Error;
use rustyscript::Runtime;
use serde_json::Value;
use tauri_plugin_svelte::ManagerExt;

#[tauri::command]
async fn handle_editor_changes(
    source_text: String,
    tab_id: String,
    app: tauri::AppHandle,
) -> Result<(), Error> {
    const STORE_NAME: &str = "storage2";
    const TABS_KEY: &str = "tabs";

    let tabs_data = app
        .svelte()
        .try_get::<Vec<Tab>>(STORE_NAME, TABS_KEY)
        .unwrap();
    let tabs = Arc::new(Mutex::new(tabs_data));

    let transformed_result = tokio::task::spawn_blocking(move || {
        let allocator = Allocator::default();
        let source_type = SourceType::ts();
        let ret = Parser::new(&allocator, &source_text, source_type).parse();

        let mut error_messages = Vec::new();
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            error_messages.push(format!("{:?}", error));
        }

        let mut runtime = Runtime::new(RuntimeOptions {
            timeout: Duration::from_millis(50),
            ..Default::default()
        }).unwrap();

        match runtime.eval::<()>(source_text.clone()) {
            Ok(_) => {}
            Err(err) => {
                error_messages.push(err.as_highlighted(Default::default()));
            }
        }

        let semantic_ret = SemanticBuilder::new().build(&ret.program);

        let ast_builder = AstBuilder::new(&allocator);

        let transformed_code = if error_messages.is_empty() {
            let mut program = allocator.alloc(ret.program);
            let ast_pass = AstReplacer::new(&allocator, source_text.clone()).build(program);
            let new_code = CodeGenerator::new()
                .with_options(CodegenOptions::default())
                .build(&program)
                .code;

            println!("{:?}", new_code);

            let mut runtime = Runtime::new(Default::default()).unwrap();

            runtime
                .eval::<()>(
                    r#"
                    globalThis.Xtal = async (line, ...valuePromise) => {
                        if (typeof globalThis.XtalResults === 'undefined') {
                            globalThis.XtalResults = [];
                        }

                        const resolvedValues = await Promise.all(valuePromise.map(async (value) => await value));

                        resolvedValues.forEach(value => {
                            globalThis.XtalResults.push({ line, value });
                        });
                    };
                "#,
                )
                .unwrap();

            match runtime.eval::<()>(new_code) {
                Ok(_) => {}
                Err(err) => {
                    error_messages.push(err.as_highlighted(Default::default()));
                }
            }

            let debug_results: Vec<Value> = match runtime.eval("globalThis.XtalResults") {
                Ok(value) => value,
                Err(err) => Vec::new(),
            };

            debug_results
        } else {
            Vec::new()
        };

        (transform_to_result(transformed_code), error_messages)
    })
    .await
    .expect("Blocking task failed");

    let mut tabs_locked = tabs.lock().await;
    if let Some(tab) = tabs_locked.iter_mut().find(|tab| tab.id == tab_id) {
        let (transformed_code, error_messages) = transformed_result;
        tab.result = transformed_code;
        tab.errors = error_messages.join("\n");
    }

    let tabs_json = serde_json::to_value(&*tabs_locked).unwrap();
    let _ = app.svelte().set(STORE_NAME, TABS_KEY, tabs_json);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(plugins::tauri_traffic_light_positioner_plugin::init())
        .plugin(tauri_plugin_svelte::init())
        .invoke_handler(tauri::generate_handler![handle_editor_changes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
