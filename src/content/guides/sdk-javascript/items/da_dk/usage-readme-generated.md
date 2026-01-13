Denne SDK giver separate indgangspunkt for browser- og servermiljøer for at sikre optimal kompatibilitet og sikkerhed:

### Browserbrug (klientside)

For browser-/frontend-applikationer, brug den browser-sikre eksport, som udelader Node.js-afhængigheder:

```typescript
// Browser-sikker import (ingen Node.js-afhængigheder)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Opret browser SDK-instans
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // valgfri, standard er https://fastcomments.com
});

// Brug offentlige API'er (ingen API-nøgle nødvendig - sikkert for browsere)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Serverbrug (Node.js)

For server-/backend-applikationer, brug det fulde SDK med SSO og autentificeringsfunktioner:

```typescript
// Server-side import (inkluderer SSO og designet til at fungere med NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Opret server SDK-instans
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Hold denne hemmelig på serveren!
  basePath: 'https://fastcomments.com' // valgfri, standard er https://fastcomments.com
});

// Brug sikrede API'er med din API-nøgle
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Kun typer-import

Hvis du kun har brug for TypeScript-typer (ingen runtime-kode), brug standardimporten:

```typescript
// Kun typer (ingen runtime-afhængigheder - sikkert overalt)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Brug af individuelle API-klasser

#### Browsermiljø

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Servermiljø  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```