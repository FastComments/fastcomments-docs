若要在使用 React 建立的網站加入留言功能，您可以在 NPM 上找到我們的 React 函式庫 <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">這裡</a>。

FastComments React 評論元件支援與 VanillaJS 相同的所有功能 - 即時評論、SSO，等等。

[inline-code-attrs-start title = '透過 NPM 的 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = '透過 Yarn 的 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

如果您位於歐盟（EU），您可能需要這樣設定 `region` 參數：

[inline-code-attrs-start title = 'React 範例 - 歐盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a> 找到 React 元件所支援的設定。

---