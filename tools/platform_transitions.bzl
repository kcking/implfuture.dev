"""
Helpers to build specific deps under a given platform without requiring
top-level --platforms flag. Uses a Starlark transition to set the target
platform for a single dependency edge.
"""

def _to_platform_impl(settings, attr):
    # attr.platform is a label selecting the desired platform target.
    # We return a dict setting the command line platforms flag for this edge.
    return {"//command_line_option:platforms": str(attr.platform)}

to_platform = transition(
    implementation = _to_platform_impl,
    inputs = [],
    outputs = ["//command_line_option:platforms"],
)

def _platform_passthrough_impl(ctx):
    # Simply pass through the file from src, but ensure it was built under
    # the transitioned platform via the cfg on the attribute.
    return [DefaultInfo(files = depset([ctx.file.src]))]

platform_passthrough = rule(
    implementation = _platform_passthrough_impl,
    attrs = {
        "src": attr.label(allow_single_file = True, cfg = to_platform),
        "platform": attr.label(mandatory = True),
    },
    doc = "Wrap a single-file target, rebuilding it under a specified platform",
)

