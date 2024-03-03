/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            "sans": ["din-2014"]
        },
    },
    plugins: [],
}