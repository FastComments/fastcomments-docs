Našo React knjižnico lahko najdete na NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">tukaj</a>.

FastComments React pripomoček za komentarje podpira vse enake funkcije kot različica VanillaJS — komentiranje v realnem času, SSO in tako naprej.

[inline-code-attrs-start title = 'FastComments React prek NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React prek Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Če ste v EU, boste želeli nastaviti parameter `region` takole:

[inline-code-attrs-start title = 'React primer - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Konfiguracijo, ki jo podpira React komponenta, lahko najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.
