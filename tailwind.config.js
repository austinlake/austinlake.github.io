module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        fontFamily: {
            'rust': ["Alfa Slab One", "serif"]
        },
        extend: {
            keyframes: {
                color_change: {
                    "0%": { "background-color": "rgb(67 56 202)" },
                    "50%": { "background-color": "rgb(225 29 72)" },
                    "100%": { "background-color": "rgb(245 158 11)" }
                },
                color_change_reverse: {
                    "0%": { "background-color": "rgb(245 158 11)" },
                    "50%": { "background-color": "rgb(225 29 72)" },
                    "100%": { "background-color": "rgb(67 56 202)" }
                }
            },
            animation: {
                color_change: "color_change 0.5s ease-in forwards",
                color_change_reverse: "color_change_reverse 0.5s ease-in forwards"
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}