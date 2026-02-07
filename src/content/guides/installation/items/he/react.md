להוספת תגובות לאתר שנבנה ב־React, ניתן למצוא את ספריית ה־React שלנו ב־NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">כאן</a>.

ווידג'ט התגובות של FastComments ל־React תומך בכל אותן תכונות כמו ה‑VanillaJS — תגובות בזמן אמת, sso, וכדומה.

[inline-code-attrs-start title = 'FastComments React דרך NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React דרך Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'דוגמה ב־React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

אם אתם באיחוד האירופי, תרצו להגדיר את הפרמטר `region` כך:

[inline-code-attrs-start title = 'דוגמה ב־React - האיחוד האירופי'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

ניתן למצוא את התצורה שהרכיב של React תומך בה <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.