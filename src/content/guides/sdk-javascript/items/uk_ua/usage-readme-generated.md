Цей SDK надає окремі точки входу для середовищ браузера та сервера, щоб забезпечити оптимальну сумісність і безпеку:

### Використання в браузері (клієнтська сторона)

Для браузерних/фронтенд-застосунків використовуйте браузерний експорт без залежностей від Node.js:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Використання на сервері (Node.js)

Для серверних/бекенд-застосунків використовуйте повний SDK зі SSO та функціями аутентифікації:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Імпорт тільки типів

Якщо вам потрібні лише типи TypeScript (без виконуваного коду), використовуйте стандартний імпорт:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Використання окремих класів API

#### Середовище браузера

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Середовище сервера  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```