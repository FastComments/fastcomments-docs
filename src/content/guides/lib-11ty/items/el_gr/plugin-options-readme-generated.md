```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Καταχώριση μόνο ενός υποσυνόλου των shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Προσθέστε ένα πρόθεμα σε κάθε καταχωρισμένο όνομα shortcode (π.χ. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```