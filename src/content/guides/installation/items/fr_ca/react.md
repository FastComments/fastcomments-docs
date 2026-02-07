Pour ajouter des commentaires à un site construit avec React, vous pouvez trouver notre bibliothèque React sur NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">ici</a>.

Le widget de commentaires FastComments pour React prend en charge toutes les mêmes fonctionnalités que celui en VanillaJS — commentaires en direct, SSO, etc.

[inline-code-attrs-start title = 'FastComments React via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Exemple React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Si vous êtes dans l'UE, vous voudrez définir le paramètre `region` comme suit :

[inline-code-attrs-start title = 'Exemple React - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Vous pouvez trouver la configuration prise en charge par le composant React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ici</a>.