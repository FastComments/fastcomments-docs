Deze SDK biedt afzonderlijke toegangspunten voor browser- en serveromgevingen om optimale compatibiliteit en beveiliging te garanderen:

### Browsergebruik (Client-Side)

Voor browser/frontend-applicaties, gebruik de browserveilige export die Node.js-afhankelijkheden uitsluit:

```typescript
// Browser-veilige import (geen Node.js-afhankelijkheden)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Maak een browser SDK-instantie
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Gebruik openbare API's (geen API-sleutel nodig - veilig voor browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Servergebruik (Node.js)

Voor server/backend-applicaties, gebruik de volledige SDK met SSO- en authenticatiefuncties:

```typescript
// Server-side import (inclusief SSO en ontworpen om te werken met NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Maak een server SDK-instantie
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Houd dit geheim op de server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Gebruik beveiligde API's met uw API-sleutel
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Alleen types importeren

Als u alleen TypeScript-typen nodig heeft (geen runtime-code), gebruik dan de standaardimport:

```typescript
// Alleen types (geen runtime-afhankelijkheden - overal veilig)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Gebruik van individuele API-klassen

#### Browseromgeving

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Serveromgeving  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```