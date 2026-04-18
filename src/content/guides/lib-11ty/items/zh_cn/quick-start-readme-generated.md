在你的 Eleventy 配置 (`.eleventy.js` 或 `eleventy.config.js`) 中注册插件：

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

或使用 ESM：

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

然后在你的模板中使用短代码。 在 Nunjucks (`.njk`)：

```njk
{% fastcomments { tenantId: "demo" } %}
```

在 Liquid (`.liquid`)：

```liquid
{% fastcomments tenantId: "demo" %}
```

将 "demo" 替换为你的 FastComments 租户 ID。