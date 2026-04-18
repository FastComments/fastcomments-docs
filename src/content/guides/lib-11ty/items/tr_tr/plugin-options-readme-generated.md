```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // Kısa kodların yalnızca bir alt kümesini kaydet:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // Kayıtlı her kısa kod adına bir önek ekle (ör. "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```