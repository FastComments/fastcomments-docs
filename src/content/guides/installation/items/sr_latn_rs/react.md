Našu React biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">ovde</a>.

FastComments React vidžet za komentare podržava sve iste funkcije kao VanillaJS verzija — komentarisanje u realnom vremenu, SSO i tako dalje.

[inline-code-attrs-start title = 'FastComments React preko NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React preko Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ako ste u EU, želećete da podesite parametar `region` ovako:

[inline-code-attrs-start title = 'React primer - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Konfiguraciju koju React komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovde</a>.
