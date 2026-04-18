```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registrar solo un subconjunto de los shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Añadir un prefijo a cada nombre de shortcode registrado (p. ej. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```