Αυτό το SDK παρέχει ξεχωριστά σημεία εισόδου για περιβάλλοντα browser και server για να εξασφαλίσει βέλτιστη συμβατότητα και ασφάλεια:

### Χρήση σε Browser (Πελάτης)

Για εφαρμογές browser/frontend, χρησιμοποίησε την ασφαλή για browser εξαγωγή που εξαιρεί τις εξαρτήσεις Node.js:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Χρήση στον Server (Node.js)

Για εφαρμογές server/backend, χρησιμοποίησε το πλήρες SDK με δυνατότητες SSO και αυθεντικοποίησης:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Εισαγωγή Μόνο Τύπων

Εάν χρειάζεσαι μόνο τύπους TypeScript (χωρίς κώδικα εκτέλεσης), χρησιμοποίησε την προεπιλεγμένη εισαγωγή:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Χρήση μεμονωμένων κλάσεων API

#### Περιβάλλον Browser

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Περιβάλλον Server  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```