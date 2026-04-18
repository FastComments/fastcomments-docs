Καταχωρήστε το plugin στο αρχείο ρυθμίσεων του Eleventy (`.eleventy.js` ή `eleventy.config.js`):

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

Έπειτα χρησιμοποιήστε τα shortcodes στα πρότυπά σας. Στο Nunjucks (`.njk`):

```njk
{% fastcomments { tenantId: "demo" } %}
```

Στο Liquid (`.liquid`):

```liquid
{% fastcomments tenantId: "demo" %}
```

Αντικαταστήστε το "demo" με το tenant ID σας στο FastComments.