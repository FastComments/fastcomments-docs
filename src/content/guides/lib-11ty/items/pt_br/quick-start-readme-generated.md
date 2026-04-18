---
Registre o plugin na sua configuração do Eleventy (`.eleventy.js` ou `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ou com ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Em seguida, use os shortcodes em seus templates. No Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

No Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Substitua `"demo"` pelo ID do seu tenant do FastComments.
---