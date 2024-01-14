/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.jinja"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["lofi", "black"],
    darkTheme: "black",
  },
};
