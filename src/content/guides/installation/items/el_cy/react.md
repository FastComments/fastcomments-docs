Για να προσθέσετε σχόλια σε έναν ιστότοπο που είναι κατασκευασμένος με React, μπορείτε να βρείτε τη βιβλιοθήκη React μας στο NPM <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">εδώ</a>.

Το στοιχείο σχολιασμού FastComments React υποστηρίζει όλες τις ίδιες δυνατότητες με την VanillaJS - live commenting, sso, και ούτω καθεξής.

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

Αν βρίσκεστε στην ΕΕ, θα θέλετε να ορίσετε την παράμετρο `region` ως εξής:

[inline-code-attrs-start title = 'Παράδειγμα React - ΕΕ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το στοιχείο React <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.