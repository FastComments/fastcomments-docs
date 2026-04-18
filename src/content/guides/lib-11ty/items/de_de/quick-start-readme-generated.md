Registrieren Sie das Plugin in Ihrer Eleventy-Konfiguration (`.eleventy.js` oder `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Oder mit ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Verwenden Sie dann die Shortcodes in Ihren Templates. In Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

In Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Ersetzen Sie `"demo"` durch Ihre FastComments Tenant-ID.