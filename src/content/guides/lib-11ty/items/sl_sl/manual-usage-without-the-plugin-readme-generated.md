Vsaka kratka koda je prav tako izvožena kot samostojna funkcija, ki vrne HTML-niz:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```