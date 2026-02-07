За додавање коментара на сајт направљен помоћу React-а, нашу React библиотеку можете наћи на NPM-у <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">овдје</a>.

FastComments React видгет за коментарисање подржава све исте функције као и VanillaJS — коментарисање уживо, SSO и тако даље.

[inline-code-attrs-start title = 'FastComments React преко NPM-а'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React преко Yarn-а'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ако се налазите у ЕУ, требало би да подесите параметар `region` на следећи начин:

[inline-code-attrs-start title = 'React пример - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Конфигурацију коју React компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овдје</a>.

---