module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        fontFamily: {
            'rust': ["Alfa Slab One", "serif"]
        },
        extend: {
            keyframes: {
                typing: {
                    from: { width: "0" },
                    to: { width: "11ch" }
                },
                color_change: {
                    "0%": { "background-color": "rgb(41, 37, 36)" },
                    "100%": { "background-color": "rgb(244, 63, 94)" }
                },
                color_change_reverse: {
                    "0%": { "background-color": "rgb(244, 63, 94)" },
                    "100%": { "background-color": "rgb(41, 37, 36)" }
                    
                },
                dark_color_change: {
                    "0%": { "background-color": "rgb(231, 229, 228)" },
                    "100%": { "background-color": "rgb(244, 63, 94)" }
                },
                dark_color_change_reverse: {
                    "0%": { "background-color": "rgb(244, 63, 94)" },
                    "100%": { "background-color": "rgb(231, 229, 228)" }
                    
                }

            },
            animation: {
                typing: "typing 2.2s steps(11)",
                color_change: "color_change 0.5s ease-in forwards",
                color_change_reverse: "color_change_reverse 0.5s ease-in forwards",
                dark_color_change: "dark_color_change 0.5s ease-in forwards",
                dark_color_change_reverse: "dark_color_change_reverse 0.5s ease-in forwards",
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}