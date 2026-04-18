Региструјте додатак у вашој Eleventy конфигурацији (`.eleventy.js` или `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Или са ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Затим користите кратке ознаке у вашим шаблонима. У Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

У Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Замените `"demo"` са вашим FastComments tenant ID.