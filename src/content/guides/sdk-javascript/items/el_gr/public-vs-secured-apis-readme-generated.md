Το SDK παρέχει τρεις κύριες κλάσεις API:

- **`DefaultApi`** - Ασφαλή endpoints που απαιτούν το κλειδί API σας για αυθεντικοποίηση. Χρησιμοποιήστε αυτά για λειτουργίες από την πλευρά του διακομιστή.
- **`PublicApi`** - Δημόσια endpoints που μπορούν να προσπελαστούν χωρίς κλειδί API. Αυτά μπορούν να κληθούν απευθείας από προγράμματα περιήγησης/κινητές συσκευές/κ.λπ.
- **`HiddenApi`** - Εσωτερικά/διαχειριστικά endpoints για προχωρημένες περιπτώσεις χρήσης.

### Παράδειγμα: Χρήση του Public API (ασφαλές για πρόγραμμα περιήγησης)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Λήψη σχολίων για μία σελίδα (δεν απαιτείται κλειδί API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Παράδειγμα: Χρήση του Default API (μόνο στον διακομιστή)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Κρατήστε αυτό μυστικό!
});
const defaultApi = new DefaultApi(config);

// Λήψη σχολίων με πλήρη πρόσβαση διαχειριστή
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```