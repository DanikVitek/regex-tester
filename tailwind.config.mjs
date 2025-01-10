import daisyui from "daisyui";

/** @type {import('tailwindcss').Config} */
export default {
    content: {
        files: ["index.html", "./src/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        }
    },
    theme: {
        extend: {},
    },
    plugins: [daisyui],
}

