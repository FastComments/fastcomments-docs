Ovaj SDK obezbeđuje odvojene ulazne tačke za browser i server okruženja kako bi se osigurala optimalna kompatibilnost i bezbednost:

### Korišćenje u pretraživaču (klijentska strana)

Za aplikacije u pretraživaču/frontend, koristite browser-bezbedan eksport koji isključuje Node.js zavisnosti:

```typescript
// Uvoz bezbedan za pretraživač (bez Node.js zavisnosti)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Kreiraj browser SDK instancu
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opciono, podrazumevano je https://fastcomments.com
});

// Koristi javne API-je (nije potreban API ključ - bezbedno za pretraživače)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Korišćenje na serveru (Node.js)

Za server/backend aplikacije, koristite kompletan SDK sa SSO i funkcijama autentifikacije:

```typescript
// Uvoz za serversku stranu (uključuje SSO i dizajniran je da radi sa NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Kreiraj server SDK instancu
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Čuvaj ovo kao tajnu na serveru!
  basePath: 'https://fastcomments.com' // opciono, podrazumevano je https://fastcomments.com
});

// Koristi zaštićene API-je sa tvojim API ključem
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Import samo tipova

Ako ti trebaju samo TypeScript tipovi (bez runtime koda), koristi podrazumevani import:

```typescript
// Samo tipovi (bez runtime zavisnosti - bezbedno svuda)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Korišćenje pojedinačnih API klasa

#### Okruženje u pretraživaču

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