const rehypePrism = require("@mapbox/rehype-prism");

const withMDX = require("@next/mdx")({
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [],
    rehypePlugins: [rehypePrism],
    providerImportSource: "@mdx-js/react",
  },
});

/** @type {import('next').NextConfig} */
module.exports = withMDX({
  pageExtensions: ["js", "jsx", "ts", "tsx", "md", "mdx"],
  reactStrictMode: true,
  webpack: function (config, options) {
    console.error(config.output);
    //  https://github.com/vercel/next.js/issues/29362#issuecomment-932767530
    config.output.webassemblyModuleFilename =
      options.isServer && !options.dev
        ? "../static/wasm/[id].wasm"
        : "static/wasm/[id].wasm";
    config.optimization.moduleIds = "named";
    console.error(config.output);
    config.experiments = { asyncWebAssembly: true, ...config.experiments };
    return config;
  },
});
