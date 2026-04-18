Registreer de plugin in je Eleventy-config (`.eleventy.js` of `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Of met ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Gebruik daarna de shortcodes in je templates. In Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

In Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Vervang `"demo"` door je FastComments tenant-ID.