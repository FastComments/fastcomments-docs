Μπορείτε να βρείτε τη βιβλιοθήκη React μας στο NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">εδώ</a>.

Το FastComments React widget σχολίων υποστηρίζει όλες τις ίδιες δυνατότητες με την έκδοση VanillaJS - ζωντανά σχόλια, SSO και ούτω καθεξής.

[inline-code-attrs-start title = 'FastComments React μέσω NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React μέσω Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
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

Αν είστε στην EU, θα θέλετε να ορίσετε την παράμετρο `region` ως εξής:

[inline-code-attrs-start title = 'Παράδειγμα React - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το React component <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.
