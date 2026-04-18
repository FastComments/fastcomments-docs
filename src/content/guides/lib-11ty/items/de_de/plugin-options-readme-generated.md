```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registriere nur eine Teilmenge der Shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Füge jedem registrierten Shortcode-Namen ein Präfix hinzu (z. B. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```