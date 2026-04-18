Зарегистрируйте плагин в конфигурации Eleventy (`.eleventy.js` или `eleventy.config.js`):

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

Затем используйте шорткоды в ваших шаблонах. В Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

В Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Замените "demo" на ваш FastComments tenant ID.