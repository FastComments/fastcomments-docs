```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // N'enregistrer qu'un sous-ensemble des shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Ajouter un préfixe à chaque nom de shortcode enregistré (p. ex. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```