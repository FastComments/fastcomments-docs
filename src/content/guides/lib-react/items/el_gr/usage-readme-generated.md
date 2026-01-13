### Το κύριο στοιχείο του Widget

Το στοιχείο FastCommentsCommentWidget περιέχει το ζωντανό widget σχολίων του FastComments.

Αντικαταστήστε "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

Το widget υποστηρίζει πολλές επιλογές - δείτε FastCommentsCommentWidgetConfig στο src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ενημέρωση της τρέχουσας σελίδας (Για SPAs)
Για να ενημερώσετε τη σελίδα/άρθρο με την οποία είναι συνδεδεμένο το νήμα σχολίων, πρέπει να ενημερώσετε τις παραμέτρους διαμόρφωσης "urlId" και "url".
Δείτε το παράδειγμα και την εξήγηση [εδώ](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Περιοχή Λογαριασμού (ΠΡΟΣΟΧΗ: Πελάτες ΕΕ)

Αν βρίσκεστε στην ΕΕ, θα πρέπει να ενημερώσετε τα client widgets για την περιοχή σας. Δείτε [examples/example-eu](/examples/example-eu/src/App.tsx);
Διαφορετικά, δεν χρειάζεται να ορίσετε `region`.

### Το widget μέτρησης σχολίων

Το στοιχείο FastCommentsCommentCountWidget περιέχει το ζωντανό widget μέτρησης σχολίων του FastComments.

Αντικαταστήστε "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

Δείτε FastCommentsCommentCountConfig στο src/index.tsx για τις υποστηριζόμενες επιλογές διαμόρφωσης.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Εγγενής

Για μια εντελώς εγγενή υλοποίηση του FastComments, δείτε [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Για ένα wrapper React Native αυτής της βιβλιοθήκης, που χρησιμοποιεί webview, δείτε [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).