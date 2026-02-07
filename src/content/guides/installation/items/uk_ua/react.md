Для додавання коментарів на сайт, створений з React, ви можете знайти нашу React-бібліотеку на NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">тут</a>.

Компонент коментування FastComments для React підтримує всі ті ж функції, що й для VanillaJS — живі коментарі, sso тощо.

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

Якщо ви перебуваєте в ЄС, варто встановити параметр `region` таким чином:

[inline-code-attrs-start title = 'Приклад React - ЄС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Ви можете знайти конфігурацію, яку підтримує компонент React, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.

---