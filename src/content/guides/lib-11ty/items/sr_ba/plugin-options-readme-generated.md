```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Региструј само подскуп shortcodes-а:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Додај префикс сваком регистрованом имену shortcode-а (нпр. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```