use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::{error::AnyError, futures::FutureExt, op2, resolve_path, Extension, Op, RuntimeOptions, Snapshot};
use std::rc::Rc;

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[op2(async)]
async fn op_set_timeout(#[number] delay: u64) -> Result<(), AnyError> {
    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
    Ok(())
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(&self, specifier: &str, referrer: &str, _kind: deno_core::ResolutionKind) -> Result<deno_core::ModuleSpecifier, AnyError> {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(&self, module_specifier: &deno_core::ModuleSpecifier, _maybe_referrer: Option<&reqwest::Url>, _is_dyn_import: bool) -> std::pin::Pin<Box<deno_core::ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        async move {
            let path = module_specifier.to_file_path().unwrap();

            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => (deno_core::ModuleType::JavaScript, false),
                MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::TypeScript | MediaType::Mts | MediaType::Cts | MediaType::Dts | MediaType::Dmts | MediaType::Dcts | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::Json => (deno_core::ModuleType::Json, false),
                _ => panic!("Unknown extension {:?}", path.extension()),
            };

            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.to_string(),
                    text_info: SourceTextInfo::from_string(code),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                parsed.transpile(&Default::default())?.text
            } else {
                code
            };
            let module = deno_core::ModuleSource::new(module_type, deno_core::ModuleCode::from(code), &module_specifier);
            Ok(module)
        }
        .boxed_local()
    }
}

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/JS_SNAPSHOT.bin"));

pub async fn run_js(code: &'static str) -> Result<(), AnyError> {
    let extension = Extension {
        name: "js",
        ops: std::borrow::Cow::Borrowed(&[op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL, op_set_timeout::DECL]),
        ..Default::default()
    };

    let mut context = deno_core::JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
        extensions: vec![extension],
        ..Default::default()
    });

    let main_module = resolve_path("<python>", &std::env::current_dir()?)?;
    let mod_id = context.load_main_module(&main_module, Some(deno_core::FastString::Static(code))).await?;
    let result = context.mod_evaluate(mod_id);

    context.run_event_loop(false).await?;
    result.await?
}

pub fn eval_js(code: &'static str) -> Result<String, AnyError> {
    let extension = Extension {
        name: "js",
        ops: std::borrow::Cow::Borrowed(&[op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL, op_set_timeout::DECL]),
        ..Default::default()
    };

    let mut context = deno_core::JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
        extensions: vec![extension],
        ..Default::default()
    });

    let global = context.execute_script("<anon>", deno_core::FastString::Static(code)).unwrap();
    let mut scope = context.handle_scope();
    let value = global.open(&mut scope);

    Ok(value.to_rust_string_lossy(&mut scope))
}
