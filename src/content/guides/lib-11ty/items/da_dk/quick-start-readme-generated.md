Registrer plugin'et i din Eleventy-konfiguration (`.eleventy.js` eller `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Eller med ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Brug derefter shortcodes i dine skabeloner. I Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

I Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Erstat `"demo"` med dit FastComments tenant ID.