"emsdk toolchain (currently only contains wasm-opt)"

def _emsdk_toolchain_impl(ctx):
    wasmopt = [f for f in ctx.attr.linker_files.files.to_list() if f.path.endswith("/bin/wasm-opt") or f.path.endswith("/bin/wasm-opt.exe")]
    if len(wasmopt) != 1:
        fail("expected 1 bin/wasm-opt file, got %s", wasmopt)
    toolchain_info = platform_common.ToolchainInfo(
        emsdkinfo = EmsdkInfo(
            wasmopt = wasmopt[0],
            linker_files = ctx.attr.linker_files,
        ),
    )
    return [toolchain_info]

emsdk_toolchain = rule(
    implementation = _emsdk_toolchain_impl,
    attrs = {
        "linker_files": attr.label(),
    },
)

EmsdkInfo = provider(
    doc = "Information about how to invoke the wasmopt compiler.",
    fields = ["wasmopt", "linker_files"],
)

def _wasmopt_impl(ctx):
    tc = ctx.toolchains["//emsdk:toolchain_type"]
    info = tc.emsdkinfo
    wasm_srcs = [f for f in ctx.attr.src.files.to_list() if f.path.endswith(".wasm")]
    if len(wasm_srcs) != 1:
        fail("expected 1 wasm file, got %s" % wasm_srcs)
    wasm_src = wasm_srcs[0]

    ctx.actions.run(
        inputs = [wasm_src],
        outputs = [ctx.outputs.out],
        executable = info.wasmopt,
        arguments = ["-Os", wasm_src.path, "-o", ctx.outputs.out.path],
    )

wasmopt = rule(
    implementation = _wasmopt_impl,
    toolchains = ["//emsdk:toolchain_type"],
    attrs = {
        "src": attr.label(allow_files = True),
        "out": attr.output(mandatory = True),
    },
)

TOOLCHAINS = {
    "mac_arm64": {
        "exec": [
            "@platforms//os:macos",
            "@platforms//cpu:arm64",
        ],
    },
    "mac": {
        "exec": [
            "@platforms//os:macos",
            "@platforms//cpu:x86_64",
        ],
    },
    "linux": {
        "exec": [
            "@platforms//os:linux",
            "@platforms//cpu:x86_64",
        ],
    },
    "win": {
        "exec": [
            "@platforms//os:windows",
            "@platforms//cpu:x86_64",
        ],
    },
}

# buildifier: disable=unused-variable
def declare_toolchains(name):
    for tc in TOOLCHAINS:
        emsdk_toolchain(
            name = "emsdk_{tc}".format(tc = tc),
            linker_files = "@emscripten_bin_{tc}//:linker_files".format(tc = tc),
        )
        native.toolchain(
            name = "emsdk_{tc}_toolchain".format(tc = tc),
            toolchain = ":emsdk_{tc}".format(tc = tc),
            exec_compatible_with = TOOLCHAINS[tc]["exec"],
            toolchain_type = "//emsdk:toolchain_type",
        )

# buildifier: disable=unnamed-macro
def register_toolchains():
    native.register_toolchains(
        *["//emsdk:emsdk_{tc}_toolchain".format(tc = tc) for tc in TOOLCHAINS]
    )
