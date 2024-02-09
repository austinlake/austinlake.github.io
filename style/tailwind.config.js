module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        fontFamily: {
            'rust': ["Alfa Slab One", "serif"]
        },
        extend: {
            keyframes: {
				fade: {
					from: { opacity: 0 },
					to: { opacity: 1 }
				},
				typing: {
                    from: { "width": "0" },
                    to: { "width": "11ch" }
                },
                twinkle: {
                    "0%": {
                        ":nth-child(1)": { opacity: 0 },
                        ":nth-child(2)": { opacity: 0.25 },
                        ":nth-child(3)": { opacity: 0.5 },
                        ":nth-child(4)": { opacity: 0.75 },
                        ":nth-child(5)": { opacity: 1 },
                    },
                    "20%": { 
                        ":nth-child(1)": { opacity: 0.25 },
                        ":nth-child(2)": { opacity: 0.5 },
                        ":nth-child(3)": { opacity: 0.75 },
                        ":nth-child(4)": { opacity: 1 },
                        ":nth-child(5)": { opacity: 0 }
                    },
                    "40%": {
                        ":nth-child(1)": { opacity: 0.5 },
                        ":nth-child(2)": { opacity: 0.75 },
                        ":nth-child(3)": { opacity: 0.1 },
                        ":nth-child(4)": { opacity: 0 },
                        ":nth-child(5)": { opacity: 0.25 }
                    },
                    "60%": {
                        ":nth-child(1)": { opacity: 0.75 },
                        ":nth-child(2)": { opacity: 1 },
                        ":nth-child(3)": { opacity: 0 },
                        ":nth-child(4)": { opacity: 0.25 },
                        ":nth-child(5)": { opacity: 0.5 }
                    },
                    "80%": {
                        ":nth-child(1)": { opacity: 1 },
                        ":nth-child(2)": { opacity: 0 },
                        ":nth-child(3)": { opacity: 0.25 },
                        ":nth-child(4)": { opacity: 0.5 },
                        ":nth-child(5)": { opacity: 0.75 }
                    },
                    "100%": {
                        ":nth-child(1)": { opacity: 0 },
                        ":nth-child(2)": { opacity: 0.25 },
                        ":nth-child(3)": { opacity: 0.5 },
                        ":nth-child(4)": { opacity: 0.75 },
                        ":nth-child(5)": { opacity: 1 }
                    },
                }
            },
            animation: {
				fade: "fade 5s linear forward",
                typing: "typing 2s steps(22)",
                twinkle: "twinkle 60s linear infinite",
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}
