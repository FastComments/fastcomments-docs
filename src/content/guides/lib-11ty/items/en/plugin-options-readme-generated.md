```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Only register a subset of the shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Add a prefix to every registered shortcode name (e.g. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```