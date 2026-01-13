Našu React biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">ovdje</a>.

FastComments React widget za komentare podržava sve iste značajke kao VanillaJS verzija — komentiranje u stvarnom vremenu, SSO i tako dalje.

[inline-code-attrs-start title = 'FastComments React putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React putem Yarna'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Ako ste u EU, željet ćete postaviti parametar `region` ovako:

[inline-code-attrs-start title = 'React primjer - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Konfiguraciju koju React komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.
