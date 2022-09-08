""" 
conditionally register zig toolchains based on host
currently zig-cc fails to compile for the host target for various reasons
"""

# NOTE: these are Java constants since this is a repository_rule

TOOLCHAIN_TO_OS = {
    "@zig_sdk//toolchain:linux_amd64_gnu.2.19": struct(os = "linux", arch = "amd64"),
    "@zig_sdk//toolchain:linux_arm64_gnu.2.28": struct(os = "linux", arch = "aarch64"),
    "@zig_sdk//toolchain:darwin_amd64": struct(os = "mac os x", arch = "x86_64"),
    "@zig_sdk//toolchain:darwin_arm64": struct(os = "mac os x", arch = "aarch64"),
    "@zig_sdk//toolchain:windows_amd64": struct(os = "windows", arch = "amd64"),
    "@zig_sdk//toolchain:windows_arm64": struct(os = "windows", arch = "aarch64"),
}

def _zig_toolchains_impl(repository_ctx):
    os = repository_ctx.os.name
    arch = repository_ctx.os.arch
    toolchains = [tc for tc in TOOLCHAIN_TO_OS if not (os.find(getattr(TOOLCHAIN_TO_OS[tc], "os")) != -1 and getattr(TOOLCHAIN_TO_OS[tc], "arch") == arch)]

    print(os)
    print(arch)
    print(toolchains)

    repository_ctx.file("toolchains.bzl", """
def register_toolchains():
    native.register_toolchains("{toolchains}")
    
    """.format(toolchains = '", "'.join(toolchains)))

    repository_ctx.file("BUILD", "")
    repository_ctx.file("WORKSPACE", "")

zig_register_toolchains_repository = repository_rule(
    implementation = _zig_toolchains_impl,
    local = True,
)
