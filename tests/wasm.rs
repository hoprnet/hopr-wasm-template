#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

/// All integration tests for WASM wrappers go in this directory.

use {{crate_name}}::my_module::wasm::*;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_foo() {
    assert_eq!(42, foo());
}
