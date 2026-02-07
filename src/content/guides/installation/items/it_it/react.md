Per aggiungere commenti a un sito creato con React, puoi trovare la nostra libreria React su NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">qui</a>.

Il widget di commenti FastComments per React supporta tutte le stesse funzionalità di quello per VanillaJS - commenti in tempo reale, sso, e così via.

[inline-code-attrs-start title = 'FastComments React tramite NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React tramite Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Esempio React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Se ti trovi nell'UE, vorrai impostare il parametro `region` in questo modo:

[inline-code-attrs-start title = 'Esempio React - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Puoi trovare la configurazione supportata dal componente React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.

---