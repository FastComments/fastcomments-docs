SDK pruža tri glavne API klase:

- **`DefaultApi`** - Zaštićeni endpointi koji zahtevaju vaš API ključ za autentifikaciju. Koristite ih za operacije na serverskoj strani.
- **`PublicApi`** - Javni endpointi kojima se može pristupiti bez API ključa. Mogu se pozivati direktno iz pregledača/mobilnih uređaja/itd.
- **`HiddenApi`** - Interni/admin endpointi za napredne slučajeve upotrebe.

### Primer: Korišćenje Public API-ja (bezbedno za pregledač)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Preuzmi komentare za stranicu (nije potreban API ključ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primer: Korišćenje Default API-ja (samo na serverskoj strani)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Držite ovo u tajnosti!
});
const defaultApi = new DefaultApi(config);

// Preuzmi komentare sa punim administratorskim pristupom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```