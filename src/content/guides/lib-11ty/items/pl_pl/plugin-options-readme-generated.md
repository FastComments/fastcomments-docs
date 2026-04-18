```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Zarejestruj tylko podzbiór shortcodów:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Dodaj prefiks do każdej zarejestrowanej nazwy shortcode'a (np. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```