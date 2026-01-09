Αυτό το SDK παρέχει ξεχωριστά σημεία εισόδου για περιβάλλοντα προγράμματος περιήγησης και διακομιστή για να εξασφαλίσει βέλτιστη συμβατότητα και ασφάλεια:

### Χρήση στο πρόγραμμα περιήγησης (Πλευρά πελάτη)

Για εφαρμογές browser/frontend, χρησιμοποιήστε την έκδοση ασφαλή για browser που εξαιρεί εξαρτήσεις Node.js:

```typescript
// Ασφαλή εισαγωγή για πρόγραμμα περιήγησης (χωρίς εξαρτήσεις Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Δημιουργία στιγμιότυπου SDK για πρόγραμμα περιήγησης
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // προαιρετικό, προεπιλογή https://fastcomments.com
});

// Χρήση δημόσιων API (δεν απαιτείται API key - ασφαλές για προγράμματα περιήγησης)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Χρήση στο διακομιστή (Node.js)

Για εφαρμογές server/backend, χρησιμοποιήστε το πλήρες SDK με λειτουργίες SSO και πιστοποίησης:

```typescript
// Εισαγωγή για διακομιστή (περιλαμβάνει SSO και έχει σχεδιαστεί για να λειτουργεί με NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Δημιουργία στιγμιότυπου SDK για διακομιστή
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Διατηρήστε αυτό μυστικό στον διακομιστή!
  basePath: 'https://fastcomments.com' // προαιρετικό, προεπιλογή https://fastcomments.com
});

// Χρήση ασφαλών API με το API key σας
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Εισαγωγή μόνο τύπων

Αν χρειάζεστε μόνο τύπους TypeScript (χωρίς κώδικα χρόνου εκτέλεσης), χρησιμοποιήστε την προεπιλεγμένη εισαγωγή:

```typescript
// Μόνο τύποι (χωρίς εξαρτήσεις χρόνου εκτέλεσης - ασφαλές παντού)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Χρήση μεμονωμένων κλάσεων API

#### Περιβάλλον προγράμματος περιήγησης

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Περιβάλλον διακομιστή  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```