您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">这里</a>找到我们的React库。

FastComments React评论小部件支持与VanillaJS版本相同的所有功能——实时评论、SSO等。

[inline-code-attrs-start title = 'FastComments React（通过NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React（通过Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

如果您在欧盟，您需要像这样设置`region`参数：

[inline-code-attrs-start title = 'React示例 - 欧盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a>找到React组件支持的配置。
