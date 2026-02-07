За додавање коментара на сајт направљен уз React, нашу React библиотеку можете пронаћи на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">овде</a>.

FastComments React коментарски видгет подржава све исте функције као и VanillaJS — уживо коментарисање, SSO, и тако даље.

[inline-code-attrs-start title = 'FastComments React преко NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React преко Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Ако се налазите у ЕУ, треба да подесите параметар `region` на следећи начин:

[inline-code-attrs-start title = 'React Пример - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Конфигурацију коју React компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.

---