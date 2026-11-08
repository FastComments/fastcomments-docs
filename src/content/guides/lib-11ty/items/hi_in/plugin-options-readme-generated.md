```js
eleventyConfig.addPlugin(fastcommentsPlugin, {
    // केवल शॉर्टकोड का एक सबसेट रजिस्टर करें:
    shortcodes: ['fastcomments', 'fastcommentsCommentCount'],
    // प्रत्येक रजिस्टर किए गए शॉर्टकोड नाम में एक प्रीफ़िक्स जोड़ें (जैसे "fc" -> "fcFastcomments"):
    prefix: 'fc'
});
```