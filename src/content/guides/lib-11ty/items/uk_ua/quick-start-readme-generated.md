Зареєструйте плагін у конфігурації Eleventy (`.eleventy.js` або `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Або з ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Потім використовуйте шорткоди у ваших шаблонах. У Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

У Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Замініть `"demo"` на ваш FastComments tenant ID.