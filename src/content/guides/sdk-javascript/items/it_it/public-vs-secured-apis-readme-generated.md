Lo SDK fornisce queste classi API:

- **`DefaultApi`** - Endpoint protetti che richiedono la tua API key per l'autenticazione. Usali per operazioni lato server.
- **`PublicApi`** - Endpoint pubblici accessibili senza una API key. Possono essere chiamati direttamente da browser/dispositivi mobili/etc.
- **`ModerationApi`** - Endpoint della dashboard moderatore (moderazione commenti, ban, badge, trust factor, ricerca). Autenticati tramite la sessione del moderatore; passa il parametro di query `sso` per i moderatori autenticati via SSO.
- **`HiddenApi`** - Endpoint interni/amministratore per casi d'uso avanzati.

### Esempio: Uso della Public API (sicuro per il browser)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Recupera i commenti per una pagina (non è richiesta la API key)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Esempio: Uso della Default API (solo lato server)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Mantienila segreta!
});
const defaultApi = new DefaultApi(config);

// Recupera i commenti con pieno accesso amministrativo
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Esempio: Uso della Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, ecc. */ });

// Chiamate autenticate dal moderatore (cookie di sessione, oppure passa `sso` per un moderatore autenticato via SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```