Eleventy の設定ファイル（`.eleventy.js` または `eleventy.config.js`）にプラグインを登録します:

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

または ESM を使用する場合:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

次に、テンプレートでショートコードを使用します。Nunjucks（`.njk`）では:

```njk
{% fastcomments { tenantId: "demo" } %}
```

Liquid（`.liquid`）では:

```liquid
{% fastcomments tenantId: "demo" %}
```

"demo" をあなたの FastComments テナント ID に置き換えてください。