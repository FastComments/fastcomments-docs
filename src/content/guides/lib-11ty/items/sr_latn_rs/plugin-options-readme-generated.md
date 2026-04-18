```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registruj samo podskup shortcodes-a:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Dodaj prefiks svakom registrovanom imenu shortcode-a (npr. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```