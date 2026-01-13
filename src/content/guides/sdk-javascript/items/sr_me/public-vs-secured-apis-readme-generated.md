SDK pruža tri glavne API klase:

- **`DefaultApi`** - Zaštićeni endpointi koji zahtijevaju vaš API ključ za autentifikaciju. Koristite ih za serverske operacije.
- **`PublicApi`** - Javni endpointi kojima se može pristupiti bez API ključa. Mogu se pozivati direktno iz preglednika/mobilnih uređaja/itd.
- **`HiddenApi`** - Interni/administratorski endpointi za napredne slučajeve upotrebe.

### Primjer: Korištenje Public API (sigurno za preglednik)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Preuzmi komentare za stranicu (nije potreban API ključ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primjer: Korištenje Default API (samo na serverskoj strani)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Čuvajte ovo u tajnosti!
});
const defaultApi = new DefaultApi(config);

// Preuzmi komentare sa punim administratorskim pristupom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```