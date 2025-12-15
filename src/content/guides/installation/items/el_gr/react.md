Μπορείτε να βρείτε τη βιβλιοθήκη React μας στο NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">εδώ</a>.

[inline-code-attrs-start title = 'FastComments React μέσω NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'
import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]
