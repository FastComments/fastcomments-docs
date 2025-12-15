Ви можете знайти нашу бібліотеку React на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">тут</a>.

Віджет коментарів FastComments для React підтримує всі ті ж функції, що й VanillaJS версія — коментування в реальному часі, SSO тощо.

[inline-code-attrs-start title = 'FastComments React через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Приклад React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Якщо ви знаходитесь у ЄС, вам потрібно встановити параметр `region` таким чином:

[inline-code-attrs-start title = 'Приклад React - ЄС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Ви можете знайти конфігурацію, яку підтримує компонент React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.
