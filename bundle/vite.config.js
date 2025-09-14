import { defineConfig } from 'vitest/config'
import prism from 'vite-plugin-prismjs'

// https://vitejs.dev/config/
export default defineConfig({
    base: '/',
    build: {
        manifest: true
    },
    plugins: [
        prism({
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
                // 'line-numbers', 'show-language'
                "autoloader",
            ],
            theme: 'okaidia',
            css: true,
        })
    ],
    test: {
        globals: true,
        css: true,
        reporters: ['verbose']
    },
})
