```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Региструјте само подмножину shortcodes-а:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Додај префикс сваком регистрованом имену shortcode-а (нпр. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```