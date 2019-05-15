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

TODO

### TODO

* [ ] Write an AST out as straw proposal text format.

* [ ] Validate and canonicalize ASTs.

* [ ] Read a straw proposal binary encoding.

* [ ] Write a straw proposal binary encoding.

* [ ] Support optional `type=` and `idx=` named parameters in the text format.

