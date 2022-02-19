const rehypePrism = require("@mapbox/rehype-prism");

const withMDX = require("@next/mdx")({
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [],
    rehypePlugins: [rehypePrism],
    providerImportSource: "@mdx-js/react",
  },
});

const { withPlausibleProxy } = require("next-plausible");

/** @type {import('next').NextConfig} */
module.exports = withPlausibleProxy(
  withMDX({
    pageExtensions: ["js", "jsx", "ts", "tsx", "md", "mdx"],
    reactStrictMode: true,
    webpack: function (config, options) {
      //  https://github.com/vercel/next.js/issues/29362#issuecomment-932767530
      config.output.webassemblyModuleFilename =
        options.isServer && !options.dev
          ? "../static/wasm/[id].wasm"
          : "static/wasm/[id].wasm";
      config.optimization.moduleIds = "named";
      config.experiments = { asyncWebAssembly: true, ...config.experiments };
      return config;
    },
  })
);
