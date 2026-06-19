Το SDK παρέχει τις ακόλουθες κλάσεις API:

- **`DefaultApi`** - Ασφαλή σημεία τερματισμού που απαιτούν το API key σας για αυθεντικοποίηση. Χρησιμοποιήστε αυτά για λειτουργίες από την πλευρά του διακομιστή.
- **`PublicApi`** - Δημόσια σημεία τερματισμού που μπορούν να προσπελαστούν χωρίς κλειδί API. Μπορούν να καλούνται απευθείας από περιηγητές/κινητές συσκευές/κ.λπ.
- **`ModerationApi`** - Σημεία τερματισμού για τον πίνακα διαχείρισης moderator (διαχείριση σχολίων, αποκλεισμοί, διακριτικά, δείκτης εμπιστοσύνης, αναζήτηση). Επαληθεύονται από τη συνεδρία του moderator· περάστε το query param `sso` για moderators που έχουν αυθεντικοποιηθεί μέσω SSO.
- **`HiddenApi`** - Εσωτερικά/διαχειριστικά σημεία τερματισμού για προχωρημένες περιπτώσεις χρήσης.

### Παράδειγμα: Χρήση του Public API (ασφαλές για περιηγητή)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Ανάκτηση σχολίων για μια σελίδα (δεν απαιτείται κλειδί API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Παράδειγμα: Χρήση του Default API (μόνο από την πλευρά του διακομιστή)

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

### Παράδειγμα: Χρήση του Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, κ.λπ. */ });

// Κλήσεις αυθεντικοποιημένες ως moderator (cookie συνεδρίας, ή περάστε `sso` για moderator με SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```