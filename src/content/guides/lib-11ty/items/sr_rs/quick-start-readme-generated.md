Registrujte dodatak u vašoj Eleventy konfiguraciji (`.eleventy.js` ili `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ili sa ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Zatim koristite shortcode-ove u vašim šablonima. U Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

U Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Zamenite `"demo"` svojim FastComments tenant ID-jem.