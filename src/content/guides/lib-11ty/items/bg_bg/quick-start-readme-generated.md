Регистрирайте плъгина в конфигурацията на Eleventy (`.eleventy.js` или `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Или с ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

След това използвайте shortcodes в шаблоните си. В Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

В Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Заменете ` "demo" ` с вашия FastComments tenant ID.