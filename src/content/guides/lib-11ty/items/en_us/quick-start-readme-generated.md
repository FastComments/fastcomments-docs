Register the plugin in your Eleventy config (`.eleventy.js` or `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Or with ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Then use the shortcodes in your templates. In Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

In Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Replace `"demo"` with your FastComments tenant ID.