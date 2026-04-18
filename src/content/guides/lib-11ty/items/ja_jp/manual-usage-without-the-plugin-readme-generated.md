各ショートコードは、HTML文字列を返すスタンドアロン関数としてもエクスポートされます:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```