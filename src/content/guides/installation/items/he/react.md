אתה יכול למצוא את ספריית ה-React שלנו ב-NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">כאן</a>.

ווידג'ט התגובות של FastComments ל-React תומך בכל אותן התכונות של גרסת VanillaJS - תגובות בזמן אמת, SSO, ועוד.

[inline-code-attrs-start title = 'FastComments React דרך NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React דרך Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'דוגמת React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

אם אתה באיחוד האירופי, תרצה להגדיר את הפרמטר `region` כך:

[inline-code-attrs-start title = 'דוגמת React - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

אתה יכול למצוא את ההגדרות שהקומפוננטה של React תומכת בהן <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.
