load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("//emsdk:emsdk.bzl", "wasmopt")

package(
    default_visibility = ["//:__subpackages__"],
)

rust_binary(
    name = "app",
    srcs = ["src/bin/app.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":gaia",
        "@rules_rust//wasm_bindgen/3rdparty:wasm_bindgen",
    ],
)

rust_library(
    name = "gaia",
    srcs = glob(
        include = [
            "src/**/*.rs",
        ],
        exclude = ["src/bin/**"],
    ),
    aliases = aliases(),
    compile_data = glob(["src/**/*.mdx"]),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_wasm_bindgen(
    name = "app_wasm",
    target = "web",
    wasm_file = ":app",
)

filegroup(
    name = "static_files",
    srcs = glob(["static/**"]) + [
        ":tailwind",
        ":copybundletostatic",
    ],
)

genrule(
    name = "tailwind",
    srcs = glob(["src/**/*.rs"]) + ["tailwind.config.js"],
    outs = ["static/tailwind.css"],
    cmd = "node bazel-out/host/bin/external/root_npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    cmd_bat = "node bazel-out/host/bin/external/root_npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    tools = ["@root_npm//tailwindcss"],
    visibility = ["//:__pkg__"],
)

genrule(
    name = "copybundletostatic",
    srcs = ["//bundle"],
    outs = ["static/bundle.js"],
    cmd = "cp $(@D)/../bundle/bundle.js $(OUTS)",
    cmd_bat = "copy \"$(@D)\\..\\bundle\\bundle.js\" $(OUTS)",
)

wasmopt(
    name = "app_wasm_opt",
    src = ":app_wasm",
    out = "app_wasm_bg_opt.wasm",
)

genrule(
    name = "app_wasm_opt_br",
    srcs = [":app_wasm_opt"],
    outs = ["app_wasm_bg_opt.wasm.br"],
    cmd = "./bazel-out/host/bin/external/brotli/brotli -9 $<",
    tools = ["@brotli"],
)
