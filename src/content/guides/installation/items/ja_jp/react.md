Reactで構築されたサイトにコメントを追加するには、NPMで当社のReactライブラリを <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">こちら</a> で見つけることができます。

FastComments の React コメントウィジェットは、VanillaJS のものと同じすべての機能（ライブコメント、SSO など）をサポートします。

[inline-code-attrs-start title = 'NPM経由の FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Yarn経由の FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

EUにいる場合は、`region` パラメータを次のように設定してください:

[inline-code-attrs-start title = 'React の例 - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Reactコンポーネントがサポートする設定は <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a> で確認できます。

---