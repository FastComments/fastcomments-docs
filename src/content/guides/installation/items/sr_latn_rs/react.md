Za dodavanje komentara na sajt napravljen sa React, možete pronaći našu React biblioteku na NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">ovde</a>.

FastComments React komentatorski widget podržava sve iste funkcije kao i VanillaJS — komentarisanje uživo, sso, i tako dalje.

[inline-code-attrs-start title = 'FastComments React preko NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React preko Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Primer za React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Ako se nalazite u EU, treba da podesite `region` parametar ovako:

[inline-code-attrs-start title = 'Primer za React - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Možete pronaći konfiguraciju koju React komponenta podržava <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovde</a>.

---