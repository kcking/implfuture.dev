load("@rules_rust_wasm_bindgen//:defs.bzl", "rust_wasm_bindgen_toolchain")

# Point the toolchain at the binary that crate_universe generated.
# The default alias name for the binary is ":wasm_bindgen_cli__bin".
rust_wasm_bindgen_toolchain(
    name = "wasm_bindgen_toolchain_impl",
    wasm_bindgen_cli = "@bindeps//:wasm-bindgen-cli__wasm-bindgen",
)

toolchain(
    name = "wasm_bindgen_toolchain",
    toolchain = ":wasm_bindgen_toolchain_impl",
    toolchain_type = "@rules_rust_wasm_bindgen//:toolchain_type",
)
