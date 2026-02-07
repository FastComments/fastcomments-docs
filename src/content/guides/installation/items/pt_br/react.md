Para adicionar comentários a um site construído com React, você pode encontrar nossa biblioteca React no NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">aqui</a>.

O widget de comentários FastComments para React suporta todos os mesmos recursos do de VanillaJS - comentários em tempo real, SSO, e assim por diante.

[inline-code-attrs-start title = 'FastComments React pelo NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React pelo Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Exemplo em React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Se você estiver na UE, defina o parâmetro `region` assim:

[inline-code-attrs-start title = 'Exemplo em React - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Você pode encontrar a configuração que o componente React suporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aqui</a>.

---