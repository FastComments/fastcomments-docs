Aby dodać komentarze do witryny zbudowanej w React, naszą bibliotekę React można znaleźć na NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">tutaj</a>.

Widżet komentowania FastComments dla React obsługuje wszystkie te same funkcje co wersja VanillaJS - komentowanie na żywo, sso i inne.

[inline-code-attrs-start title = 'FastComments React przez NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React przez Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Przykład React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Jeśli jesteś w UE, ustaw parametr `region` w następujący sposób:

[inline-code-attrs-start title = 'Przykład React - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Możesz znaleźć konfigurację, którą obsługuje komponent React, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.

---