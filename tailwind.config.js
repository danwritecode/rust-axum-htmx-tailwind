/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./ui/templates/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

