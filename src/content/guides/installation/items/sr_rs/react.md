За додавање коментара на сајт направљен са React-ом, нашу React библиотеку можете пронаћи на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">овде</a>.

FastComments React виџет за коментаре подржава све исте функције као VanillaJS верзија - коментаре у реалном времену, sso и тако даље.

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

Ако сте у ЕУ, треба да подесите `region` параметар на следећи начин:

[inline-code-attrs-start title = 'React пример - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Конфигурацију коју React компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.