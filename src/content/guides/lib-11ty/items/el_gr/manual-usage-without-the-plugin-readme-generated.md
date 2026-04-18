Κάθε shortcode εξάγεται επίσης ως ανεξάρτητη συνάρτηση που επιστρέφει τη συμβολοσειρά HTML:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```