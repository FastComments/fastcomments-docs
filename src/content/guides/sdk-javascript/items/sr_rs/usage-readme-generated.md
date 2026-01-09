Овај SDK обезбеђује одвојене почетне тачке за прегледач и серверске средине како би се осигурала оптимална компатибилност и безбедност:

### Коришћење у прегледачу (Client-Side)

За апликације у прегледачу/frontend, користите browser-safe export који искључује Node.js зависности:

```typescript
// Импорт прилагођен прегледачу (нема Node.js зависности)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Креирање browser SDK инстанце
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // опционално, подразумевано https://fastcomments.com
});

// Користите јавне API-је (API кључ није потребан - безбедно за прегледаче)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Коришћење на серверу (Node.js)

За серверске/backend апликације, користите пун SDK који садржи SSO и аутентификационе функције:

```typescript
// Импорт за сервер (укључује SSO и дизајниран да ради са NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Креирање server SDK инстанце
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Чувајте ово као тајну на серверу!
  basePath: 'https://fastcomments.com' // опционално, подразумевано https://fastcomments.com
});

// Користите заштићене API-је са вашим API кључем
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Увоз само типова

Ако су вам потребни само TypeScript типови (без runtime кода), користите подразумевани импорт:

```typescript
// Само типови (без runtime зависности - безбедно свуда)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Коришћење појединачних API класа

#### Окружење прегледача

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Серверско окружење  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```