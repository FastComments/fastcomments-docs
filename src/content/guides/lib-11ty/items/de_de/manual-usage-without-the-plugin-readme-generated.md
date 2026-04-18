Jeder Shortcode wird außerdem als eigenständige Funktion exportiert, die den HTML-String zurückgibt:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```