For adding comments to a site built with React, you can find our React library on NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">burada</a>.

The FastComments React commenting widget supports all of the same features of the VanillaJS one - canlı yorum, sso ve benzeri.

[inline-code-attrs-start title = 'FastComments React NPM üzerinden'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Yarn üzerinden'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Eğer AB'deyseniz, `region` parametresini şöyle ayarlamak isteyebilirsiniz:

[inline-code-attrs-start title = 'React Örneği - AB'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

You can find the configuration the React component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a>.

---