/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		extend: {
			colors: {
				'ct-pink': '#f5c2e7',
				'ct-red': '#f38ba8',
				'ct-base': '#1e1e2e',
			}
		},
	},
	plugins: [],
}
