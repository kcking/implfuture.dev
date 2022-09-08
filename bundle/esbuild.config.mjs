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
