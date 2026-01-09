---
De SDK biedt drie hoofd-API-klassen:

- **`DefaultApi`** - Beveiligde endpoints die uw API-sleutel vereisen voor authenticatie. Gebruik deze voor serverzijde-bewerkingen.
- **`PublicApi`** - Publieke endpoints die kunnen worden benaderd zonder API-sleutel. Deze kunnen rechtstreeks vanuit browsers/mobiele apparaten/etc. worden aangeroepen.
- **`HiddenApi`** - Interne/admin endpoints voor geavanceerde gebruikssituaties.

### Voorbeeld: Public API gebruiken (browser-veilig)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Haal reacties op voor een pagina (geen API-sleutel vereist)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Voorbeeld: Default API gebruiken (alleen serverzijde)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Houd dit geheim!
});
const defaultApi = new DefaultApi(config);

// Haal reacties op met volledige beheerdersrechten
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```
---