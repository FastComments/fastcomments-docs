Questo SDK fornisce punti di ingresso separati per gli ambienti browser e server per garantire compatibilità e sicurezza ottimali:

### Browser Usage (Client-Side)

Per applicazioni browser/frontend, utilizzare l'export sicuro per il browser che esclude le dipendenze di Node.js:

```typescript
// Import sicuro per il browser (nessuna dipendenza Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Crea un'istanza dello SDK per il browser
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opzionale, predefinito https://fastcomments.com
});

// Usa API pubbliche (nessuna API key necessaria - sicuro per i browser)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Server Usage (Node.js)

Per applicazioni server/backend, utilizzare lo SDK completo con funzionalità SSO e di autenticazione:

```typescript
// Import lato server (include SSO e progettato per funzionare con NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Crea un'istanza dello SDK per il server
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Mantieni questo segreto sul server!
  basePath: 'https://fastcomments.com' // opzionale, predefinito https://fastcomments.com
});

// Usa API protette con la tua API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Import Solo Tipi

Se hai bisogno solo dei tipi TypeScript (nessun codice a runtime), utilizza l'importazione di default:

```typescript
// Solo tipi (nessuna dipendenza runtime - sicuro ovunque)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Using Individual API Classes

#### Browser Environment

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Server Environment  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```