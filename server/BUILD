load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load(
    "@io_bazel_rules_docker//container:container.bzl",
    "container_image",
    "container_push",
)
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

rust_binary(
    name = "server",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm",
        "//:static_files",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ) + ["//:gaia"],
)

rust_binary(
    name = "opt",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm",
        # wasm-opt takes ~3s to run, only use in release build
        "//:app_wasm_opt",
        "//:app_wasm_opt_br",
        "//:static_files",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_env = {
        "APP_WASM_PATH": "/app_wasm_bg_opt.wasm",
        "AXUM_PRECOMPRESSED_WASM": "1",
    },
    deps = all_crate_deps(
        normal = True,
    ) + ["//:gaia"],
)

platform(
    name = "linux_amd64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

platform(
    name = "linux_arm64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:arm64",
    ],
)

pkg_tar(
    name = "opt_tar",
    srcs = [":opt"],
    include_runfiles = True,
    package_dir = "/app",
    # keeps relative path consistent for static files in both dev and release
    strip_prefix = "/server",
)

container_image(
    name = "image-arm64",
    architecture = "arm64",
    # References container_pull from WORKSPACE (above)
    base = "@cc_base//image",
    cmd = ["/app/opt"],
    env = {
        "HTTP_LISTEN_ADDR": "0.0.0.0:8080",
    },
    tars = [":opt_tar"],
    workdir = "/app",
)

container_image(
    name = "image-amd64",
    architecture = "amd64",
    # References container_pull from WORKSPACE (above)
    base = "@cc_base//image",
    cmd = ["/app/opt"],
    env = {
        "HTTP_LISTEN_ADDR": "0.0.0.0:8080",
    },
    tars = [":opt_tar"],
    workdir = "/app",
)

container_push(
    name = "push-amd64",
    format = "Docker",
    image = ":image-amd64",
    registry = "us-west1-docker.pkg.dev",
    repository = "implfuture/implfuture/server",
    tag = "latest",
)
