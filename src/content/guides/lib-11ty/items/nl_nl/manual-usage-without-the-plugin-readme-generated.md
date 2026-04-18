Elke shortcode wordt ook geëxporteerd als een op zichzelf staande functie die de HTML-string retourneert:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```