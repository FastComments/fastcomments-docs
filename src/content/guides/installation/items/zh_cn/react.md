要在使用 React 构建的网站上添加评论，您可以在 NPM 上找到我们的 React 库 <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">这里</a>。

FastComments 的 React 评论组件支持与 VanillaJS 相同的所有功能——实时评论、sso 等。

[inline-code-attrs-start title = '通过 NPM 使用 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = '通过 Yarn 使用 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

如果您位于欧盟，请像下面这样设置 `region` 参数：

[inline-code-attrs-start title = 'React 示例 - 欧盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a> 找到 React 组件支持的配置。

---