The SDK παρέχει τρεις κύριες κλάσεις API:

- **`DefaultApi`** - Secured endpoints that require your API key for authentication. Use these for server-side operations.
- **`PublicApi`** - Public endpoints that can be accessed without an API key. These can be called directly from browsers/mobile devices/etc.
- **`HiddenApi`** - Internal/admin endpoints for advanced use cases.

### Παράδειγμα: Χρήση Public API (ασφαλές για browsers)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Λήψη σχολίων για σελίδα (δεν απαιτείται API key)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Παράδειγμα: Χρήση Default API (μόνο στην πλευρά του διακομιστή)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Κρατήστε το μυστικό!
});
const defaultApi = new DefaultApi(config);

// Λήψη σχολίων με πλήρη διαχειριστική πρόσβαση
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```