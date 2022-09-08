/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        display: ["Major Mono Display", "sans-serif"],
        body: ["Raleway", "sans-serif"],
      },
    },
  },
  plugins: [],
};
