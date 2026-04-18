```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // 단축 코드의 하위 집합만 등록합니다:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // 등록된 모든 단축 코드 이름에 접두사를 추가합니다 (예: "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```