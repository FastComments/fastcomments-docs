अपनी Eleventy कॉन्फ़िग (`.eleventy.js` या `eleventy.config.js`) में प्लगइन रजिस्टर करें:

```js
const { fastcommentsPlugin } = require('fastcomments-11ty');

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
};
```

या ESM के साथ:

```js
import { fastcommentsPlugin } from 'fastcomments-11ty';

export default function(eleventyConfig) {
    eleventyConfig.addPlugin(fastcommentsPlugin);
}
```

फिर अपने टेम्पलेट्स में शॉर्टकोड का उपयोग करें। Nunjucks (`.njk`) में:

```njk
{% fastcomments { tenantId: "demo" } %}
```

Liquid (`.liquid`) में:

```liquid
{% fastcomments tenantId: "demo" %}
```

`"demo"` को अपनी FastComments टेनेंट ID से बदलें।