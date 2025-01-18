load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

workspace(name = "implfuture.dev")


# for tailwind / esbuild bundling
http_archive(
    name = "build_bazel_rules_nodejs",
    sha256 = "f10a3a12894fc3c9bf578ee5a5691769f6805c4be84359681a785a0c12e8d2b6",
    urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-5.5.3.tar.gz"],
)

# local_repository(
#     name = "build_bazel_rules_nodejs",
#     path = "../../dev/rules_nodejs",
# )

load("@build_bazel_rules_nodejs//:repositories.bzl", "build_bazel_rules_nodejs_dependencies")

build_bazel_rules_nodejs_dependencies()

load("@build_bazel_rules_nodejs//:index.bzl", "node_repositories", "yarn_install")

node_repositories()

yarn_install(
    name = "app_npm",
    package_json = "//bundle:package.json",
    yarn_lock = "//bundle:yarn.lock",
)

yarn_install(
    name = "root_npm",
    package_json = "//:package.json",
    yarn_lock = "//:yarn.lock",
)

load("@build_bazel_rules_nodejs//toolchains/esbuild:esbuild_repositories.bzl", "esbuild_repositories")

# based on my testing esbuild only works in root npm repository
esbuild_repositories(npm_repository = "root_npm")

# for wasm-opt
http_archive(
    name = "emsdk",
    sha256 = "189149281f36ca8edfad2632aa7c3a028bb116b7dbb967c4dac2f6496f60702c",
    strip_prefix = "emsdk-3.1.19/bazel",
    url = "https://github.com/emscripten-core/emsdk/archive/refs/tags/3.1.19.tar.gz",
)

load("@emsdk//:deps.bzl", emsdk_deps = "deps")

emsdk_deps()

load("@emsdk//:emscripten_deps.bzl", emsdk_emscripten_deps = "emscripten_deps")

emsdk_emscripten_deps(emscripten_version = "3.1.19")

HERMETIC_CC_TOOLCHAIN_VERSION = "v2.1.2"

http_archive(
    name = "hermetic_cc_toolchain",
    sha256 = "28fc71b9b3191c312ee83faa1dc65b38eb70c3a57740368f7e7c7a49bedf3106",
    urls = [
        "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
        "https://github.com/uber/hermetic_cc_toolchain/releases/download/{0}/hermetic_cc_toolchain-{0}.tar.gz".format(HERMETIC_CC_TOOLCHAIN_VERSION),
    ],
)

load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")

# Plain zig_toolchains() will pick reasonable defaults. See
# toolchain/defs.bzl:toolchains on how to change the Zig SDK version and
# download URL.

zig_toolchains()

# compression for large wasm bundle
git_repository(
    name = "brotli",
    commit = "ed738e842d2fbdf2d6459e39267a633c4a9b2f5d",  # 2023-08-29
    remote = "https://github.com/google/brotli",
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "f4a9314518ca6acfa16cc4ab43b0b8ce1e4ea64b81c38d8a3772883f153346b8",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.50.1/rules_go-v0.50.1.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.50.1/rules_go-v0.50.1.zip",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.23.0")

# gazelle needed for container_push rule
http_archive(
    name = "bazel_gazelle",
    sha256 = "5982e5463f171da99e3bdaeff8c0f48283a7a5f396ec5282910b9e8a49c0dd7e",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
    ],
)


load("//emsdk:emsdk.bzl", register_wasmopt_toolchains = "register_toolchains")

register_wasmopt_toolchains()

load("//zig:zig.bzl", "zig_register_toolchains_repository")

zig_register_toolchains_repository(name = "zig_register")

load("@zig_register//:toolchains.bzl", register_zig_toolchains = "register_toolchains")

register_zig_toolchains()
