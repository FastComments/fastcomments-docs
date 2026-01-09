L'SDK fornisce tre principali classi API:

- **`DefaultApi`** - Endpoint protetti che richiedono la tua chiave API per l'autenticazione. Usali per operazioni lato server.
- **`PublicApi`** - Endpoint pubblici accessibili senza una chiave API. Possono essere chiamati direttamente da browser/dispositivi mobili/etc.
- **`HiddenApi`** - Endpoint interni/amministrativi per casi d'uso avanzati.

### Esempio: Uso della Public API (sicuro per il browser)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Ottieni i commenti per una pagina (non è richiesta la chiave API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Esempio: Uso della Default API (solo lato server)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Tienila segreta!
});
const defaultApi = new DefaultApi(config);

// Ottieni i commenti con accesso amministrativo completo
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```