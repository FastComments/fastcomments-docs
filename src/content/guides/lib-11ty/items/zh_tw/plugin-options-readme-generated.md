```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // 只註冊部分短代碼：
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // 為每個註冊的短代碼名稱新增前綴（例如 "fc" -> "fcFastcomments"）：
    prefix: 'fc'
});
```