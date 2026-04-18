Registra el plugin en la configuración de Eleventy (`.eleventy.js` o `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

O con ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Luego usa los shortcodes en tus plantillas. En Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

En Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Reemplaza `"demo"` con tu FastComments tenant ID.