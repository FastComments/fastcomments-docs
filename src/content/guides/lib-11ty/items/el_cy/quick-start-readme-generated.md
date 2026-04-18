Καταχωρήστε το πρόσθετο στη ρύθμιση του Eleventy (`.eleventy.js` ή `eleventy.config.js`):

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

Ή με ESM:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

Στη συνέχεια χρησιμοποιήστε τα shortcodes στα πρότυπά σας. Στο Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

Στο Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Αντικαταστήστε το `"demo"` με το tenant ID του λογαριασμού σας στο FastComments.