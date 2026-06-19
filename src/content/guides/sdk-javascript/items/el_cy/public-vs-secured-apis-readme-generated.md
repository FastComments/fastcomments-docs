Το SDK παρέχει τις εξής κλάσεις API:

- **`DefaultApi`** - Προστατευμένα endpoints που απαιτούν το API key σας για αυθεντικοποίηση. Χρησιμοποιήστε αυτά για λειτουργίες στην πλευρά του server.
- **`PublicApi`** - Δημόσια endpoints που είναι προσβάσιμα χωρίς API key. Μπορούν να κληθούν απευθείας από προγράμματα περιήγησης/κινητές συσκευές/κ.λπ.
- **`ModerationApi`** - Endpoints του πίνακα ελέγχου moderator (διαχείριση σχολίων, αποκλεισμοί, εμβλήματα, δείκτης εμπιστοσύνης, αναζήτηση). Αυθεντικοποιούνται μέσω της συνεδρίας του moderator· περάστε την παράμετρο ερωτήματος `sso` για moderators που έχουν αυθεντικοποίηση μέσω SSO.
- **`HiddenApi`** - Εσωτερικά/διαχειριστικά endpoints για προχωρημένες χρήσεις.

### Παράδειγμα: Χρήση του Public API (ασφαλές για προγράμματα περιήγησης)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Λήψη σχολίων για μια σελίδα (δεν απαιτείται API key)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Παράδειγμα: Χρήση του Default API (μόνο στην πλευρά του server)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Κράτησέ το μυστικό!
});
const defaultApi = new DefaultApi(config);

// Λήψη σχολίων με πλήρη πρόσβαση διαχειριστή
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Παράδειγμα: Χρήση του Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, κ.λπ. */ });

// Κλήσεις αυθεντικοποιημένες ως moderator (session cookie, ή πέρασε το `sso` για moderator που έχει αυθεντικοποίηση μέσω SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```