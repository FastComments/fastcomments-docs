Можете да намерите нашата React библиотека в NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">тук</a>.

FastComments React уиджетът за коментари поддържа всички същите функции като VanillaJS версията — коментиране в реално време, SSO и т.н.

[inline-code-attrs-start title = 'FastComments React чрез NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React чрез Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ако сте в ЕС, ще искате да зададете параметъра `region` по следния начин:

[inline-code-attrs-start title = 'React пример - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Можете да намерите конфигурацията, която React компонентът поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.
