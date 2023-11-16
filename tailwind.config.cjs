/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        greenAccent: { 500: "#60f78e", 600: "#9dfa42" },
        darkAccent: { 500: "#9e9e9e", 600: "#18181b" },
        gradientAccent: { 500: "linear-gradient(195.89deg, #9dfa42 5.24%, #60f78e 88.92% );"} ,
      },
    },
  },
  plugins: [],
};
