Chaque shortcode est également exporté en tant que fonction autonome qui renvoie la chaîne HTML :

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```