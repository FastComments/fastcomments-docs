在你的 Eleventy 設定檔（`.eleventy.js` 或 `eleventy.config.js`）中註冊此外掛：

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

然後在你的模板中使用短碼。在 Nunjucks（`.njk`）：

```njk
{% fastcomments { tenantId: "demo" } %}
```

在 Liquid（`.liquid`）：

```liquid
{% fastcomments tenantId: "demo" %}
```

將 `"demo"` 替換為你的 FastComments 租戶 ID。