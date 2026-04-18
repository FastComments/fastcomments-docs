Her kısa kod ayrıca HTML dizesini döndüren bağımsız bir fonksiyon olarak dışa aktarılır:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```