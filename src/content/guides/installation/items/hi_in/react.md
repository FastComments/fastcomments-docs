React के साथ बनाई गई साइट पर कमेंट्स जोड़ने के लिए, हमारी React लाइब्रेरी NPM पर <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">यहाँ</a> उपलब्ध है।

FastComments React commenting widget VanillaJS वाले की सभी सुविधाओं को सपोर्ट करता है - live commenting, sso, और भी बहुत कुछ।

[inline-code-attrs-start title = 'NPM के माध्यम से FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Yarn के माध्यम से FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

यदि आप EU में हैं, तो आप `region` पैरामीटर इस प्रकार सेट करना चाहेंगे:

[inline-code-attrs-start title = 'React उदाहरण - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

React कंपोनेंट द्वारा सपोर्टेड कॉन्फ़िगरेशन आप <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">यहाँ</a> देख सकते हैं।