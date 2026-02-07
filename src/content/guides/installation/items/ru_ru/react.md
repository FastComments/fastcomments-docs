Для добавления комментариев на сайт, построенный с использованием React, вы можете найти нашу библиотеку React на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">здесь</a>.

Виджет комментариев FastComments для React поддерживает все те же возможности, что и версия на VanillaJS — живые комментарии, sso и т.д.

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

Если вы находитесь в ЕС, вам следует установить параметр `region` следующим образом:

[inline-code-attrs-start title = 'Пример React - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Вы можете найти конфигурацию, которую поддерживает React-компонент, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">здесь</a>.

---