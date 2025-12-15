ReactライブラリはNPMの<a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">こちら</a>で見つけることができます。

FastComments Reactコメントウィジェットは、VanillaJS版と同じすべての機能（ライブコメント、SSO など）をサポートしています。

[inline-code-attrs-start title = 'FastComments React（NPM経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React（Yarn経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Reactの例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

EUにいる場合は、次のように`region`パラメータを設定する必要があります：

[inline-code-attrs-start title = 'Reactの例 - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Reactコンポーネントがサポートする設定は<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a>で見つけることができます。
