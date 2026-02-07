За додавање коментара на сајт изграђен помоћу React-а, нашу React библиотеку можете пронаћи на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">овде</a>.

The FastComments React commenting widget supports all of the same features of the VanillaJS one - коментарисање уживо, sso, и тако даље.

[inline-code-attrs-start title = 'FastComments React преко NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React преко Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

If you're in the EU, you'll want to set the `region` parameter like so:

[inline-code-attrs-start title = 'React пример - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

You can find the configuration the React component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.