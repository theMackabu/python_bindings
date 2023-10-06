use deno_core::extension;
use std::env;
use std::path::PathBuf;

fn main() {
    extension!(engine, js = ["src/js/core.js", "src/js/helpers.js"]);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("JS_SNAPSHOT.bin");

    let _snapshot = deno_core::snapshot_util::create_snapshot(deno_core::snapshot_util::CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        snapshot_path,
        startup_snapshot: None,
        extensions: vec![engine::init_ops_and_esm()],
        compression_cb: None,
        with_runtime_cb: None,
    });
}
