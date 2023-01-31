
// This type will be made available to both WASM (when the "wasm" feature is turned on)
// and non-WASM (pure Rust)
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct MyStruct {
    foo: u32,        // private members do not need to have WASM-compatible types
    pub bar: u32,    // public members MUST have WASM-compatible types, if struct used with #[cfg_attr(feature = "wasm"...
    // NOTE: you can use #[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen(skip))]
    // on a public attribute that shall not be available to WASM
}

// This function won't be available in WASM, even when the "wasm" feature is on
pub fn foo() -> u32 { 42 }

// This function must not use WASM-specific types, but must be WASM compatible,
// because the attribute makes it available to both WASM and non-WASM (pure Rust)
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn bar() -> u32 { 0 }

impl MyStruct {
   // Here, specify methods with types that are strictly NOT compatible with WASM (pure Rust)
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl MyStruct {
   // Here, specify methods with types that are compatible with BOTH WASM and non-WASM (pure Rust)
}

// Trait implementations for types can NEVER be made available for WASM
impl std::string::ToString for ChannelStatus {
    fn to_string(&self) -> String {
        todo!()
    }
}

/// Unit tests of pure Rust code (must not contain anything WASM specific)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(42, foo());
    }
}

/// Module for WASM-specific Rust code
#[cfg(feature = "wasm")]
pub mod wasm {

    // Specify free functions here that operate on MyStruct which are stricly WASM-only compatible (e.g. use js_sys,...etc.)
    // Also types and other free functions that are WASM-only specific are defined here

    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;

    #[wasm_bindgen]
    pub fn foo(_val: JsValue) -> i32 {
        super::foo()
    }
}

