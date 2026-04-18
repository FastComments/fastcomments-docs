```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registrer kun en delmængde af shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Tilføj et præfiks til hvert registreret shortcode-navn (f.eks. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```