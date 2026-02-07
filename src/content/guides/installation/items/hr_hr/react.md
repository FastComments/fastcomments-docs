Za dodavanje komentara na web-stranicu izrađenu s Reactom, našu React biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">ovdje</a>.

FastComments React widget za komentiranje podržava sve iste značajke kao i VanillaJS - komentiranje uživo, sso, i tako dalje.

[inline-code-attrs-start title = 'FastComments React putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React putem Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Primjer za React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

Ako se nalazite u EU, trebali biste postaviti parametar `region` ovako:

[inline-code-attrs-start title = 'Primjer za React - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Konfiguraciju koju React komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.

---