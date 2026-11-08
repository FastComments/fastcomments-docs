प्रत्येक शॉर्टकोड को एक स्टैंडअलोन फ़ंक्शन के रूप में भी एक्सपोर्ट किया जाता है जो HTML स्ट्रिंग लौटाता है:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```