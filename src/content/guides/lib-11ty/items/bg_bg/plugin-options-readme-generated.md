```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Регистрирайте само подмножество от shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Добавете префикс към всяко регистрирано име на shortcode (напр. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```