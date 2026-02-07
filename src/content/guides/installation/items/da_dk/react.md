---
For at tilføje kommentarer til et site bygget med React kan du finde vores React-bibliotek på NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">her</a>.

FastComments React-kommentar-widgetten understøtter alle de samme funktioner som VanillaJS-udgaven - live-kommentering, sso og så videre.

[inline-code-attrs-start title = 'FastComments React via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React-eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Hvis du er i EU, bør du sætte `region`-parameteren således:

[inline-code-attrs-start title = 'React-eksempel - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Du kan finde den konfiguration, som React-komponenten understøtter, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.

---