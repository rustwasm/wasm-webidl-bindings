# `wasm-webidl-bindings`


**Read, write, and manipulate the Wasm WebIDL bindings custom section.**

[![](https://docs.rs/wasm-webidl-bindings/badge.svg)](https://docs.rs/wasm-webidl-bindings/)
[![](https://img.shields.io/crates/v/wasm-webidl-bindings.svg)](https://crates.io/crates/wasm-webidl-bindings)
[![](https://img.shields.io/crates/d/wasm-webidl-bindings.svg)](https://crates.io/crates/wasm-webidl-bindings)

### What's Inside

* A parser for the straw proposal text format. See `crates/text-parser/src/grammar.lalrpop`.

* A set of AST types for representing and manipulating WebIDL bindings. See
  `src/ast.rs`.

* An encoder and decoder for the straw proposal binary format. See the
  implementation at `src/binary/encode.rs` and details on the format at
  `BINARY.md`.

### Example

#### Parsing the Text Format and Encoding it in the Binary Format

```rust
#[cfg(feature = "text")]
use wasm_webidl_bindings::{binary, text};

// Get the `walrus::Module` that this webidl-bindings section is for.
//
// The Wasm type and func that are being bound are:
//
//     (type $EncodeIntoFuncWasm
//       (param anyref anyref i32 i32)
//       (result i64 i64))
//
//     (func $encodeInto
//       (import "TextEncoder" "encodeInto")
//       (type $EncodeIntoFuncWasm))
let raw_wasm: Vec<u8> = get_wasm_buffer_from_somewhere();

let mut config = walrus::ModuleConfig::default();

// Register a function to run after the module is parsed, but with access to the
// mapping from indices in the original Wasm binary to their newly assigned
// walrus IDs.
//
// This is where we will parse the Web IDL bindings text.
config.on_parse(|module, indices_to_ids| {
    let webidl_bindings = text::parse(module, indices_to_ids, r#"
        type $TextEncoderEncodeIntoResult
            (dict
                (field "read" unsigned long long)
                (field "written" unsigned long long))

        type $EncodeIntoFuncWebIDL
            (func (method any)
                (param USVString Uint8Array)
                (result $TextEncoderEncodeIntoResult))

        func-binding $encodeIntoBinding import $EncodeIntoFuncWasm $EncodeIntoFuncWebIDL
            (param
                (as any 0)
                (as any 1)
                (view Int8Array 2 3))
            (result
                (as i64 (field 0 (get 0)))
                (as i64 (field 1 (get 0))))

        bind $encodeInto $encodeIntoBinding
    "#)?;

    println!("The parsed Web IDL bindings are {:#?}", webidl_bindings);

    // Insert the `webidl_bindings` into the module as a custom section.
    module.customs.add(webidl_bindings);

    Ok(())
});

let mut module = config.parse(&raw_wasm)?;

// Reserialize the Wasm module along with its new Web IDL bindings
// section.
let new_raw_wasm = module.emit_wasm();
```

