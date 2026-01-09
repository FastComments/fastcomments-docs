SDK udostępnia trzy główne klasy API:

- **`DefaultApi`** - Zabezpieczone endpointy, które wymagają klucza API do uwierzytelnienia. Używaj ich do operacji po stronie serwera.
- **`PublicApi`** - Publiczne endpointy, do których można uzyskać dostęp bez klucza API. Można je wywoływać bezpośrednio z przeglądarek/urządzeń mobilnych itp.
- **`HiddenApi`** - Wewnętrzne/adminowe endpointy do zaawansowanych zastosowań.

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Pobierz komentarze dla strony (nie wymaga klucza API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Trzymaj to w tajemnicy!
});
const defaultApi = new DefaultApi(config);

// Pobierz komentarze z pełnym dostępem administratora
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```