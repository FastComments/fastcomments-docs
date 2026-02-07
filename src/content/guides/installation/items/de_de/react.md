Um Kommentare zu einer mit React erstellten Website hinzuzufügen, finden Sie unsere React-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">hier</a>.

Das FastComments React-Kommentierungs-Widget unterstützt alle der gleichen Funktionen des VanillaJS-Widgets - Live-Kommentare, SSO, und so weiter.

[inline-code-attrs-start title = 'FastComments React über NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React über Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React-Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Wenn Sie sich in der EU befinden, sollten Sie den `region`-Parameter wie folgt setzen:

[inline-code-attrs-start title = 'React-Beispiel - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Sie finden die Konfiguration, die die React-Komponente unterstützt, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.

---