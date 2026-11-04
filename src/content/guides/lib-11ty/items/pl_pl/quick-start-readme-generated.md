Zarejestruj wtyczkę w konfiguracji Eleventy (`.eleventy.js` lub `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Lub z ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Następnie użyj shortcodes w swoich szablonach. W Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

W Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Zamień `"demo"` na swój identyfikator tenant FastComments.