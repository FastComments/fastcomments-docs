```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // רשום רק תת-קבוצה של ה-shortcodes:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // הוסף קידומת לכל שם shortcode רשום (למשל "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```