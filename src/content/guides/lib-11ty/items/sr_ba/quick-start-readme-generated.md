Registrujte plugin u vašoj Eleventy konfiguraciji (`.eleventy.js` ili `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ili koristeći ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Zatim koristite shortcodes u vašim predlošcima. U Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

U Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Zamijenite `"demo"` sa vašim FastComments tenant ID.