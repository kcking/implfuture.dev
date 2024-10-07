load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@bazel_skylib//rules:common_settings.bzl", "bool_flag")
load("@rules_rust//wasm_bindgen:defs.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("//emsdk:emsdk.bzl", "wasmopt")

package(
    default_visibility = ["//:__subpackages__"],
)

config_setting(
    name = "debug",
    values = {
        "compilation_mode": "dbg",
    },
)

config_setting(
    name = "fastbuild",
    values = {
        "compilation_mode": "fastbuild",
    },
)

bool_flag(
    name = "show_drafts",
    build_setting_default = False,
)

rust_binary(
    name = "app",
    srcs = ["src/bin/app.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_flags = select({
        ":debug": [
            "-Copt-level=0",
        ],
        ":fastbuild": [],
        "//conditions:default": [
            "-Clto",
            "-Ccodegen-units=1",
            "-Cpanic=abort",
            "-Copt-level=z",
        ],
    }),
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":implfuture",
        "@rules_rust//wasm_bindgen/3rdparty:wasm_bindgen",
    ],
)

rust_library(
    name = "implfuture",
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
    rustc_env = select({
        ":show_drafts_config": {
            "SHOW_UNPUBLISHED": "1",
        },
        "//conditions:default": {},
    }),
    deps = all_crate_deps(
        normal = True,
    ),
)

config_setting(
    name = "show_drafts_config",
    flag_values = {
        "//:show_drafts": "1",
    },
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
    cmd = "$(execpath @root_npm//tailwindcss/bin:tailwindcss) --output=$(OUTS)",
    tools = ["@root_npm//tailwindcss/bin:tailwindcss"],
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
    cmd = "$(execpath @brotli) -9 $<",
    tools = ["@brotli"],
)
