각 숏코드는 HTML 문자열을 반환하는 독립 실행형 함수로도 내보내집니다:

```js
const { fastcomments, commentCount } = require('fastcomments-11ty');

eleventyConfig.addShortcode('comments', fastcomments);
eleventyConfig.addShortcode('commentCount', commentCount);
```