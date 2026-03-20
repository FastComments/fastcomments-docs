### Το κύριο Widget Component

Το στοιχείο FastCommentsCommentWidget περιέχει το ζωντανό widget σχολίων FastComments.

Αντικαταστήστε το "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [here](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

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

### Ενημέρωση της τρέχουσας σελίδας (για SPAs)
Για να ενημερώσετε τη σελίδα/άρθρο στο οποίο συνδέεται το νήμα σχολίων, πρέπει να ενημερώσετε τις παραμέτρους διαμόρφωσης "urlId" και "url".
Δείτε το παράδειγμα και την εξήγηση [εδώ](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Περιοχή λογαριασμού (ΠΡΟΣΟΧΗ: Πελάτες ΕΕ)

Αν βρίσκεστε στην ΕΕ, θα θελήσετε να ενημερώσετε τα client widgets για την περιοχή στην οποία βρίσκεστε. Δείτε [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Διαφορετικά, δεν χρειάζεται να ορίσετε το `region`.

### Το Widget με τον μετρητή σχολίων

Το στοιχείο FastCommentsCommentCountWidget περιέχει το ζωντανό widget μετρητή σχολίων FastComments.

Αντικαταστήστε το "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [here](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

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

Για μια εντελώς native υλοποίηση του FastComments, δείτε [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Για ένα wrapper React Native αυτής της βιβλιοθήκης, που χρησιμοποιεί webview, δείτε [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).