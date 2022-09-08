load("@root_npm//@bazel/typescript:index.bzl", "ts_project")
load("@root_npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")

ts_project(
    name = "tsproject",
    srcs = [
        "app.ts",
    ],
    deps = ["@app_npm//prismjs"],
)

esbuild(
    name = "bundle",
    config = ":esbuild_config",
    entry_point = "app.ts",
    visibility = ["//:__pkg__"],
    deps = [
        ":tsproject",
    ],
)

esbuild_config(
    name = "esbuild_config",
    config_file = "esbuild.config.mjs",
    deps = [
        "@root_npm//esbuild",
        "@root_npm//esbuild-plugin-prismjs",
    ],
)
