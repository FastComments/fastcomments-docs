Enregistrez le plugin dans votre configuration Eleventy (`.eleventy.js` ou `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ou avec ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Ensuite, utilisez les shortcodes dans vos modèles. Dans Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

Dans Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Remplacez "demo" par votre ID de locataire FastComments.