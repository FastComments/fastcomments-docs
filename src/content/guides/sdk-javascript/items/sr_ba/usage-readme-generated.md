Овај SDK обезбјеђује одвојене тачке улаза за прегледач и серверска окружења како би се осигурала оптимална компатибилност и сигурност:

### Коришћење у прегледачу (на клијентској страни)

За апликације у прегледачу (frontend), користите извоз намјењен прегледачу који искључује зависности Node.js:

```typescript
// Увоз намјењен прегледачу (без зависности Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Креирај инстанцу SDK за прегледач
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // опционално, по подразумевању је https://fastcomments.com
});

// Користи јавне API-је (API кључ није потребан — безбједно за прегледаче)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Коришћење на серверу (Node.js)

За серверске (backend) апликације, користите пун SDK са SSO и функцијама аутентификације:

```typescript
// Увоз за сервер (укључује SSO и дизајниран да ради са NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Креирај инстанцу серверског SDK-а
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Чувај ово у тајности на серверу!
  basePath: 'https://fastcomments.com' // опционално, по подразумевању је https://fastcomments.com
});

// Користи заштићене API-је са твојим API кључем
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Увоз само типова

Ако ти требају само TypeScript типови (без кода при извршавању), користи подразумевани увоз:

```typescript
// Само типови (без зависности при извршавању - безбједно у сваком окружењу)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Коришћење појединачних API класа

#### Прегледачко окружење

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