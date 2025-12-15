React kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">burada</a> bulabilirsiniz.

FastComments React yorum widget'ı, VanillaJS olanla aynı tüm özellikleri destekler - canlı yorumlama, SSO vb.

[inline-code-attrs-start title = 'FastComments React (NPM ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React (Yarn ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

AB'deyseniz, `region` parametresini şu şekilde ayarlamanız gerekir:

[inline-code-attrs-start title = 'React Örneği - AB'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

React bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.
