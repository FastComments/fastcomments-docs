Para añadir comentarios a un sitio construido con React, puedes encontrar nuestra librería React en NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">aquí</a>.

El widget de comentarios FastComments para React admite todas las mismas funciones que el de VanillaJS: comentarios en vivo, sso, y demás.

[inline-code-attrs-start title = 'FastComments React vía NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React vía Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Ejemplo en React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Si te encuentras en la UE, querrás establecer el parámetro `region` así:

[inline-code-attrs-start title = 'Ejemplo React - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Puedes encontrar la configuración que admite el componente React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.

---