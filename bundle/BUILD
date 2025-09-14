load("@app_npm//:defs.bzl", "npm_link_all_packages")
load("@app_npm//bundle:tailwindcss/package_json.bzl", tailwindcss = "bin")

# use tsconfig-to-swcconfig from react npm package
load("@app_npm//bundle:tsconfig-to-swcconfig/package_json.bzl", tsconfig_to_swcconfig = "bin")
load("@app_npm//bundle:vite/package_json.bzl", "bin")
load("@aspect_rules_js//js:defs.bzl", "js_library", "js_run_binary", "js_run_devserver", "js_test")
load("@aspect_rules_ts//ts:defs.bzl", "ts_config", "ts_project")
load(":defs.bzl", "TRANSPILER")

package(default_visibility = ["//visibility:public"])

npm_link_all_packages()

ts_config(
    name = "tsconfig",
    src = "tsconfig.json",
    visibility = ["//visibility:public"],
)

tsconfig_to_swcconfig.t2s(
    name = "write_swcrc",
    srcs = ["tsconfig.json"],
    args = [
        "--filename",
        "$(location tsconfig.json)",
    ],
    stdout = ".swcrc",
    visibility = ["//visibility:public"],
)

ts_project(
    name = "src",
    srcs = ["index.ts"],
    declaration = True,
    resolve_json_module = True,
    transpiler = TRANSPILER,
    tsconfig = ":tsconfig",
    visibility = ["//bundle:__subpackages__"],
    deps = [
        ":node_modules/prismjs",
        ":node_modules/vite-plugin-prismjs",
    ],
)

js_library(
    name = "package_json",
    srcs = ["package.json"],
    visibility = ["//visibility:public"],
    deps = [],
)

js_library(
    name = "vite.config",
    srcs = ["vite.config.js"],
    data = [
        ":node_modules/vite-plugin-prismjs",
        ":node_modules/vitest",
    ],
    visibility = ["//visibility:public"],
)

bin.vite_binary(
    name = "vite",
    chdir = package_name(),
    data = ["vite.config"],
)

js_run_binary(
    name = "bundle",
    srcs = [
        "index.html",
        "tsconfig.json",
        "vite.config.js",
        ":src",
    ],
    args = ["build"],
    mnemonic = "ViteBuild",
    out_dirs = ["dist"],
    tool = ":vite",
    visibility = ["//visibility:public"],
)

tailwindcss.tailwindcss_binary(
    name = "tailwindcss",
)
