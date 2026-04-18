```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // 仅注册部分短代码：
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // 为每个注册的短代码名称添加前缀（例如 "fc" -> "fcFastcomments"）：
    prefix: 'fc'
});
```