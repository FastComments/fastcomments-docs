```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Registrirajte le podmnožico shortcodov:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Dodajte predpono vsakemu registriranemu imenu shortcoda (npr. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```