```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registriraj samo podskup shortcode-ova:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Dodaj prefiks svakom registriranom imenu shortcode-a (npr. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```