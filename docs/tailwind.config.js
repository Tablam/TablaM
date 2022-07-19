/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './themes/**/*.html',
        './templates/**/*.html',
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/typography'),
    ]
}