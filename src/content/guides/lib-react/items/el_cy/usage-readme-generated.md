### The Main Widget Component

Το στοιχείο FastCommentsCommentWidget περιέχει το ζωντανό widget σχολίων FastComments.

Αντικαταστήστε το "demo" πιο κάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

Το widget υποστηρίζει πολλές επιλογές - δείτε το FastCommentsCommentWidgetConfig στο src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Updating The Current Page (For SPAs)
Για να ενημερώσετε τη σελίδα/άρθρο στο οποίο είναι συνδεδεμένη η συζήτηση σχολίων πρέπει να ενημερώσετε τις παραμέτρους ρύθμισης "urlId" και "url".
Δείτε το παράδειγμα και την εξήγηση [εδώ](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

Αν βρίσκεστε στην ΕΕ, θα θελήσετε να δηλώσετε στα client widgets σε ποια περιοχή βρίσκεστε. Δείτε [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Σε διαφορετική περίπτωση, δεν χρειάζεται να ορίσετε το `region`.

### The Comment Count Widget

Το στοιχείο FastCommentsCommentCountWidget περιέχει το ζωντανό widget πλήθους σχολίων FastComments.

Αντικαταστήστε το "demo" πιο κάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

Δείτε το FastCommentsCommentCountConfig στο src/index.tsx για τις υποστηριζόμενες επιλογές ρύθμισης.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

Για μια εντελώς native υλοποίηση του FastComments, δείτε το [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Για ένα React Native wrapper αυτής της βιβλιοθήκης, που χρησιμοποιεί webview, δείτε το [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).