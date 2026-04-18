```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // ショートコードのサブセットのみ登録します:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // 登録される各ショートコード名にプレフィックスを追加します（例: "fc" -> "fcFastcomments"）:
    prefix: 'fc'
});
```