您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">這裡</a>找到我們的React函式庫。

FastComments React評論小工具支援與VanillaJS版本相同的所有功能——即時評論、SSO等。

[inline-code-attrs-start title = 'FastComments React（透過NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React（透過Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

如果您在歐盟，您需要像這樣設定`region`參數：

[inline-code-attrs-start title = 'React範例 - 歐盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a>找到React元件支援的配置。
