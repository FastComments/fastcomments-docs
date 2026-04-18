הוסף את התוסף לקובץ התצורה של Eleventy (`.eleventy.js` או `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

או עם ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

לאחר מכן השתמש בקיצורי-הקוד בתבניות שלך. ב-Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

ב-Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

החלף את `"demo"` ב־tenant ID של FastComments שלך.