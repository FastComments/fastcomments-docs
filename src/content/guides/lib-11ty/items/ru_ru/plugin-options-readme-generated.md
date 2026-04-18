```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Зарегистрировать только подмножество шорткодов:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Добавить префикс ко всем зарегистрированным именам шорткодов (например, "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```