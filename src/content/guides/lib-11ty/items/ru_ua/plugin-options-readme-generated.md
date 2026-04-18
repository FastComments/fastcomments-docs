```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Регистрировать только подмножество шорткодов:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Добавить префикс ко всем зарегистрированным именам шорткодов (e.g. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```