```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registra solo un sottoinsieme degli shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Aggiungi un prefisso a ogni nome di shortcode registrato (es. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```