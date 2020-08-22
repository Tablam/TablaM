module.exports = {
	future: {
    	removeDeprecatedGapUtilities: true,
	},	
  	purge: {
    	content: [
    		'./themes/**/*.html',
    		'./templates/**/*.html',
		],
	    // These options are passed through directly to PurgeCSS
	    options: {
	      whitelist: ['lead'],
	    }		
  	},	
	theme: {
    	extend: {},
	},
	variants: {},
	plugins: [
  		require('@tailwindcss/typography'),
	],
}
