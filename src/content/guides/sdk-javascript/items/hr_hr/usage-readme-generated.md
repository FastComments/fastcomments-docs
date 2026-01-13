---
Ovaj SDK pruža odvojene ulazne točke za pregledničko i serversko okruženje kako bi se osigurala optimalna kompatibilnost i sigurnost:

### Korištenje u pregledniku (klijentska strana)

Za aplikacije u pregledniku/frontend koristite pregledniku sigurni izvoz koji isključuje Node.js ovisnosti:

```typescript
// Uvoz siguran za preglednik (bez Node.js ovisnosti)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Kreirajte instancu SDK-a za preglednik
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Koristite javne API-je (nije potreban API ključ - sigurno za preglednike)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Korištenje na serveru (Node.js)

Za server/backend aplikacije, koristite puni SDK s SSO i značajkama autentikacije:

```typescript
// Uvoz za server (uključuje SSO i dizajniran za rad s NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Kreirajte instancu SDK-a za server
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Čuvajte ovo tajnim na serveru!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Koristite zaštićene API-je s vašim API ključem
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Uvoz samo tipova

Ako su vam potrebni samo TypeScript tipovi (nema runtime koda), koristite zadani uvoz:

```typescript
// Samo tipovi (bez runtime ovisnosti - sigurno svugdje)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Korištenje pojedinačnih API klasa

#### Pregledničko okruženje

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Serversko okruženje  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```
---