Questo SDK fornisce punti di ingresso separati per ambienti browser e server per garantire compatibilità e sicurezza ottimali:

### Utilizzo nel Browser (Client-Side)

Per applicazioni browser/frontend, utilizzare l'export sicuro per il browser che esclude le dipendenze Node.js:

```typescript
// Import sicuro per il browser (nessuna dipendenza Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Creare un'istanza dell'SDK per il browser
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opzionale, predefinito https://fastcomments.com
});

// Utilizzare le API pubbliche (nessuna API key necessaria - sicuro per i browser)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Utilizzo sul Server (Node.js)

Per applicazioni server/backend, utilizzare l'SDK completo con funzionalità SSO e di autenticazione:

```typescript
// Import lato server (include SSO e progettato per funzionare con NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Creare un'istanza dell'SDK per il server
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Mantenere segreta questa chiave sul server!
  basePath: 'https://fastcomments.com' // opzionale, predefinito https://fastcomments.com
});

// Utilizzare le API protette con la propria API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Import Solo dei Tipi

Se si necessita solo dei tipi TypeScript (nessun codice runtime), utilizzare l'import predefinito:

```typescript
// Solo tipi (nessuna dipendenza runtime - sicuro ovunque)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Utilizzo delle Classi API Individuali

#### Ambiente Browser

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Ambiente Server  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```