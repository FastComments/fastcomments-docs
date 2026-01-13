Овај SDK пружа одвојене улазне тачке за окружења прегледача и сервера како би обезбедио оптималну компатибилност и безбедност:

### Коришћење у прегледачу (клијентска страна)

За апликације у прегледачу/фронт‑енда, користите извоз погодан за прегледач који искључује Node.js зависности:

```typescript
// Увоз погодан за прегледач (нема Node.js зависности)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Креирај инстанцу SDK за прегледач
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // опционално, подразумева се https://fastcomments.com
});

// Користи јавне API-је (није потребан API кључ - безбедно за прегледаче)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Коришћење на серверу (Node.js)

За серверске/бекенд апликације користите пун SDK са SSO и функцијама аутентификације:

```typescript
// Увоз за сервер (укључује SSO и дизајниран да ради са NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Креирај инстанцу SDK за сервер
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Чувај ово као тајну на серверу!
  basePath: 'https://fastcomments.com' // опционално, подразумева се https://fastcomments.com
});

// Користи обезбеђене API-је са твојим API кључем
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Увоз само типова

Ако су ти потребни само TypeScript типови (нема извршног кода), користи подразумевани увоз:

```typescript
// Само типови (нема извршних зависности - безбедно свуда)
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