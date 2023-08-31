use std::io::Read;

use wasmer::{Cranelift, Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        // "/hello.wasm"
        "/wasix-axum.wasm"
    );
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read(wasm_path)?;

    let compiler = Cranelift::default();

    // Create a Store.
    let mut store = Store::new(compiler);

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    let (stdout_tx, mut stdout_rx) = Pipe::channel();

    // Run the module.
    WasiEnv::builder("hello")
        .env("PORT", "8080")
        .stdout(Box::new(stdout_tx))
        .run_with_store(module, &mut store)?;

    eprintln!("Run complete - reading output");

    let mut buf = String::new();
    stdout_rx.read_to_string(&mut buf).unwrap();

    eprintln!("Output: {buf}");

    Ok(())
}
