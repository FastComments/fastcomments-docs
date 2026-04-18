Registrirajte dodatak u svojoj Eleventy konfiguraciji (`.eleventy.js` ili `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ili s ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Zatim upotrijebite shortcode-ove u svojim predlošcima. U Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

U Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Zamijenite `"demo"` svojim FastComments tenant ID-jem.