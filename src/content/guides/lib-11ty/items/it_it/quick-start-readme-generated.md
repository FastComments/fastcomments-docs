Registra il plugin nella configurazione di Eleventy (`.eleventy.js` o `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Oppure con ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Poi usa gli shortcode nei tuoi template. In Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

In Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Sostituisci `"demo"` con il tuo tenant ID di FastComments.