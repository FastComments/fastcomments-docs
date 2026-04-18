Registrirajte vtičnik v vaši Eleventy konfiguraciji (`.eleventy.js` ali `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ali z ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Nato uporabite kratke kode v svojih predlogah. V Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

V Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Zamenjajte `"demo"` z ID-jem vašega FastComments najemnika.