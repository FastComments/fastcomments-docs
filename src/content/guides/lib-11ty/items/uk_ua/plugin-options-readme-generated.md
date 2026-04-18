```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Реєструйте лише підмножину шорткодів:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Додайте префікс до кожного імені зареєстрованого шорткоду (наприклад "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```