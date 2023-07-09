module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        fontFamily: {
            'rust': ["Alfa Slab One", "serif"]
        },
        extend: {
            keyframes: {
                typing: {
                    from: { "width": "0" },
                    to: { "width": "11ch" }
                },
                highlight: {
                    from: { "background-color": "rgb(0, 0, 0)" },
                    to: { "background-color": "rgb(255, 50, 50)" }
                },
                dark_highlight: {
                    from: { "background-color": "rgb(0, 0, 0)" },
                    to: { "background-color": "rgb(0, 155, 155)" }
                },
                falling: {
                    from: {
                        opacity: 0,
                        transform: "translateY(-50vmin)"
                    },
                    
                    to: {
                        opacity: 1,
                        transform: "translateY(0)"
                    }
                },
                fade_in: {
                    "0%": {
                        opacity: 0,
                    },
                    
                    "80%": {
                        opacity: 0,
                    },

                    "100%": {
                        opacity: 1,
                    }
                },
                spread6: {
                    "0%": {
                        width: 0,
                    },
                    "50%": {
                        width: "1ch",
                    },                          
                    "100%": {
                        width: "6ch",
                    }
                },
                spread5: {
                    "0%": {
                        width: 0,
                    },
                    "50%": {
                        width: "1ch",
                    },                          
                    "100%": {
                        width: "5ch",
                    }
                },
                spread3: {
                    "0%": {
                        width: 0,
                    },
                    "50%": {
                        width: "1ch",
                    },                          
                    "100%": {
                        width: "3ch",
                    }
                },
                spin: {
                    "100%": {
                        "transform": "rotate(360deg)",
                }
            },
            animation: {
                typing: "typing 2s steps(22)",
                highlight: "highlight 0.25s linear forwards",
                highlight_rev: "highlight 0.25s linear backwards",
                dark_highlight: "dark_highlight 0.25s linear forwards",
                dark_highlight_rev: "dark_highlight 0.25s linear backwards",
                falling: "falling 0.5s ease-in forwards",
                fade_in: "fade_in 5s ease-in forwards",
                spread6: "spread6 5s ease-in forwards",
                spread5: "spread5 5s ease-in forwards",
                spread3: "spread3 5s ease-in forwards",
                spin1: "spin 10s linear infinite",
                spin2: "spin 15s linear infinite",
                spin3: "spin 18s linear infinite",
                spin4: "spin 20s linear infinite",
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}

}