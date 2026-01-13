Вы можете найти нашу библиотеку React на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">здесь</a>.

Виджет комментариев FastComments для React поддерживает все те же функции, что и VanillaJS версия — комментирование в реальном времени, SSO и так далее.

[inline-code-attrs-start title = 'FastComments React через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Пример React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Если вы находитесь в ЕС, вам нужно установить параметр `region` следующим образом:

[inline-code-attrs-start title = 'Пример React - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Вы можете найти конфигурацию, которую поддерживает компонент React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">здесь</a>.
