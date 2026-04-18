```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Региструјте само подскуп shortcode-ова:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Додајте префикс сваком регистрованом имену shortcode-а (нпр. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```