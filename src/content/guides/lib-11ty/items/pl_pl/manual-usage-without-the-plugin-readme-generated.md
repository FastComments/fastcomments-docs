Każdy shortcode jest również eksportowany jako samodzielna funkcja, która zwraca łańcuch HTML:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```