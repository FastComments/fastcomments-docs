You can find our React library on NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">here</a>.

The FastComments React commenting widget supports all of the same features of the VanillaJS one - live commenting, sso, and so on.

[inline-code-attrs-start title = 'FastComments React via NPM'; type = 'shell'; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React via Yarn'; type = 'shell'; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React Example'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

The fastcomments-vue and fastcomments-vue-next libraries support the same configuration as the VanillaJS commenting widget.

You can find the configuration the React component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.
