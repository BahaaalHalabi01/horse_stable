/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: ({ colors }) => ({
        secondary: colors.stone[800],
        primary: colors.lime[600],
      }),
    },
  },
  plugins: [],
};
