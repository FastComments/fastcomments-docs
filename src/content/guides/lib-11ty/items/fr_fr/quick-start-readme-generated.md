Enregistrez le plugin dans votre configuration d'Eleventy (`.eleventy.js` ou `eleventy.config.js`):

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

Puis utilisez les shortcodes dans vos templates. Dans Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

Dans Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Remplacez `"demo"` par votre ID de locataire FastComments.