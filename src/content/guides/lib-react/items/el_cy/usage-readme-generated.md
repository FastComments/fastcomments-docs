### Το Κύριο Στοιχείο Widget

Το στοιχείο FastCommentsCommentWidget περιέχει το ζωντανό widget σχολίων FastComments.

Αντικαταστήστε το "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης FastComments.

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

### Ενημέρωση της Τρέχουσας Σελίδας (Για SPAs)
Για να ενημερώσετε τη σελίδα/άρθρο στο οποίο είναι συνδεδεμένο το νήμα σχολίων, πρέπει να ενημερώσετε τις παραμέτρους διαμόρφωσης "urlId" και "url".
Δείτε το παράδειγμα και την εξήγηση [εδώ](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Περιοχή Λογαριασμού (ΠΡΟΣΟΧΗ: Πελάτες της ΕΕ)

Αν βρίσκεστε στην ΕΕ, θα θελήσετε να ενημερώσετε τα client widgets για την περιοχή στην οποία βρίσκεστε. Δείτε [examples/example-eu](/examples/example-eu/src/App.tsx);
Αλλιώς, δεν χρειάζεται να ορίσετε `region`.

### Το Widget Καταμέτρησης Σχολίων

Το στοιχείο FastCommentsCommentCountWidget περιέχει το ζωντανό widget καταμέτρησης σχολίων FastComments.

Αντικαταστήστε το "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης FastComments.

Δείτε το FastCommentsCommentCountConfig στο src/index.tsx για τις υποστηριζόμενες επιλογές διαμόρφωσης.

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

Για μια εντελώς εγγενή υλοποίηση του FastComments, δείτε [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Για ένα wrapper React Native αυτής της βιβλιοθήκης, που χρησιμοποιεί webview, δείτε [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).