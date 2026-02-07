Za dodajanje komentarjev na spletno mesto, zgrajeno z React, lahko naš React paket najdete na NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">tukaj</a>.

The FastComments React commenting widget supports all of the same features of the VanillaJS one - komentiranje v živo, SSO, in tako naprej.

[inline-code-attrs-start title = 'FastComments React preko NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React preko Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Če ste v EU, boste želeli nastaviti parameter `region` tako:

[inline-code-attrs-start title = 'React Primer - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Konfiguracijo, ki jo komponenta React podpira, najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.

---