import { prismjsPlugin } from "esbuild-plugin-prismjs";

export default {
  plugins: [
    prismjsPlugin({
      inline: false,
      languages: [
        "typescript",
        "javascript",
        "css",
        "markup",
        "rust",
        "bash",
        "tsx",
        "python", // for BUILD files and starlark
        "markdown",
        "json",
      ],
      plugins: [
        // "line-highlight",
        // "line-numbers",
        // "show-language",
        // "copy-to-clipboard",
        "autoloader",
      ],
      theme: "okaidia",
      css: true,
    }),
  ],
};
