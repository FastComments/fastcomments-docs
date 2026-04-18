כל shortcode מיוצא גם כפונקציה עצמאית שמחזירה את מחרוזת ה-HTML:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```