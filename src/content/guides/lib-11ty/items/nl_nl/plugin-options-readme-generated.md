```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registreer alleen een deel van de shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Voeg een voorvoegsel toe aan elke geregistreerde shortcode-naam (bijv. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```