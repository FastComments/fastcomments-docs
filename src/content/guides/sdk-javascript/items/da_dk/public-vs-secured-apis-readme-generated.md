SDK'et leverer tre hoved-API-klasser:

- **`DefaultApi`** - Sikrede endpoints, der kræver din API-nøgle til autentificering. Brug disse til operationer på serversiden.
- **`PublicApi`** - Offentlige endpoints, som kan tilgås uden en API-nøgle. Disse kan kaldes direkte fra browsere/mobilenheder/osv.
- **`HiddenApi`** - Interne/admin-endpoints til avancerede brugssituationer.

### Eksempel: Brug af Public API (browser-sikkert)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Hent kommentarer for en side (ingen API-nøgle nødvendig)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Eksempel: Brug af Default API (kun på serversiden)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Hold denne hemmelig!
});
const defaultApi = new DefaultApi(config);

// Hent kommentarer med fuld admin-adgang
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```