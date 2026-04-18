```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Καταχώρισε μόνο ένα υποσύνολο των shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Πρόσθεσε ένα πρόθεμα σε κάθε καταχωρημένο όνομα shortcode (π.χ. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```