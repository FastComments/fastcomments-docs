Om reacties toe te voegen aan een site die met React is gebouwd, vind je onze React-bibliotheek op NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">hier</a>.

De FastComments React-commentaarwidget ondersteunt alle functies van de VanillaJS-versie - live reacties, sso, enzovoort.

[inline-code-attrs-start title = 'FastComments React via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React-voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Als je in de EU bent, wil je de `region`-parameter als volgt instellen:

[inline-code-attrs-start title = 'React-voorbeeld - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

De configuratie die de React-component ondersteunt vind je <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.

---