Eklentiyi Eleventy yapılandırmanıza (`.eleventy.js` veya `eleventy.config.js`) kaydedin:

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Veya ESM ile:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Daha sonra şablonlarınızda kısa kodları kullanın. Nunjucks (`.njk`) içinde:

```njk
{% fastcomments { tenantId: "demo" } %}
```

Liquid (`.liquid`) içinde:

```liquid
{% fastcomments tenantId: "demo" %}
```

`"demo"`'yu FastComments tenant ID'niz ile değiştirin.