SDK pruža tri glavne klase API-ja:

- **`DefaultApi`** - Zaštićene krajnje točke koje zahtijevaju vaš API ključ za autentifikaciju. Koristite ih za operacije na strani poslužitelja.
- **`PublicApi`** - Javne krajnje točke kojima se može pristupiti bez API ključa. One se mogu pozivati izravno iz preglednika/mobilnih uređaja/itd.
- **`HiddenApi`** - Interni/admin krajnje točke za napredne slučajeve upotrebe.

### Primjer: Korištenje Public API-ja (sigurno za preglednik)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Dohvati komentare za stranicu (nije potreban API ključ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Primjer: Korištenje Default API-ja (samo na strani poslužitelja)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Držite ovo u tajnosti!
});
const defaultApi = new DefaultApi(config);

// Dohvati komentare s punim administratorskim pristupom
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```