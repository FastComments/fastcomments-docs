```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registrar apenas um subconjunto dos shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Adicionar um prefixo a cada nome de shortcode registrado (por exemplo, "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```