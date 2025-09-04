load("@aspect_rules_swc//swc:defs.bzl", "swc")
load("@bazel_skylib//lib:partial.bzl", "partial")

# Partially-apply our (generated) .swcrc config file to the swc starlark function
TRANSPILER = partial.make(
    swc,
    swcrc = "//bundle:.swcrc",
)
