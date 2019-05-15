# `wasm-webidl-bindings`


**Read and write the Wasm WebIDL bindings custom section.**

[![](https://docs.rs/wasm-webidl-bindings/badge.svg)](https://docs.rs/wasm-webidl-bindings/)
[![](https://img.shields.io/crates/v/wasm-webidl-bindings.svg)](https://crates.io/crates/wasm-webidl-bindings)
[![](https://img.shields.io/crates/d/wasm-webidl-bindings.svg)](https://crates.io/crates/wasm-webidl-bindings)
[![Build Status](https://dev.azure.com/rustwasm/wasm-webidl-bindings/_apis/build/status/rustwasm.wasm-webidl-bindings?branchName=master)](https://dev.azure.com/rustwasm/wasm-webidl-bindings/_build/latest?definitionId=2&branchName=master)

### What's Inside

* A parser for the straw proposal text format. See `src/grammar.lalrpop`.

* A set of AST types for representing and manipulating WebIDL bindings. See
  `src/ast.rs`.

### Example

#### Parsing the Straw Proposal Text Format

```rust
use wasm_webidl_bindings::{ast::BuildAstActions, parser::parse};

let mut actions = BuildAstActions::default();

// The Wasm type and func that are being bound are:
//
//     (type $EncodeIntoFuncWasm
//       (param anyref anyref i32 i32)
//       (result i64 i64))
//
//     (func $encodeInto
//       (import "TextEncoder" "encodeInto")
//       (type $EncodeIntoFuncWasm))
let ast = parse(&mut actions, r#"
    type $TextEncoderEncodeIntoResult
      (dict
        (field "read" unsigned_long_long)
        (field "written" unsigned_long_long))

    type $EncodeIntoFuncWebIDL
       (func (method any)
          (param USVString Uint8Array)
          (result $TextEncoderEncodeIntoResult))

    func-binding $encodeIntoBinding import $EncodeIntoFuncWasm $EncodeIntoFuncWebIDL
      (param
        (as any 0)
        (as any 1)
        (view uint8 2 3))
      (result
        (as i64 (field 0 (get 0)))
        (as i64 (field 1 (get 0))))

    bind $encodeInto $encodeIntoBinding
"#)?;

println!("The parsed AST is {:#?}", ast);
```

### TODO

* [ ] Write an AST out as straw proposal text format.

* [ ] Validate and canonicalize ASTs.

* [ ] Read a straw proposal binary encoding.

* [ ] Write a straw proposal binary encoding.

* [ ] Support optional `type=` and `idx=` named parameters in the text format.

