Eleventy 구성 파일(`.eleventy.js` or `eleventy.config.js`)에 플러그인을 등록하세요:

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

또는 ESM 사용 시:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

그런 다음 템플릿에서 숏코드를 사용하세요. Nunjucks (`.njk`)에서는:

```njk
{% fastcomments { tenantId: "demo" } %}
```

Liquid (`.liquid`)에서는:

```liquid
{% fastcomments tenantId: "demo" %}
```

`"demo"`를 본인의 FastComments 테넌트 ID로 변경하세요.