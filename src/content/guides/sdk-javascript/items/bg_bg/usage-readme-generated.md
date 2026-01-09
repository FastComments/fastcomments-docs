Този SDK предоставя отделни входни точки за браузърни и сървърни среди, за да осигури оптимална съвместимост и сигурност:

### Използване в браузъра (клиентска страна)

За браузърни/фронтенд приложения използвайте безопасния за браузър износ, който изключва зависимости от Node.js:

```typescript
// Импорт безопасен за браузър (без зависимости от Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Създаване на екземпляр на SDK за браузър
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // по избор, по подразбиране https://fastcomments.com
});

// Използване на публични API (не се изисква API ключ - безопасно за браузъра)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Използване на сървъра (Node.js)

За сървърни/бекенд приложения използвайте пълния SDK с SSO и функции за удостоверяване:

```typescript
// Импорт за сървърната страна (включва SSO и е проектиран да работи с NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Създаване на екземпляр на SDK за сървър
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Дръжте това в тайна на сървъра!
  basePath: 'https://fastcomments.com' // по избор, по подразбиране https://fastcomments.com
});

// Използване на защитени API с вашия API ключ
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Импорт само на типове

Ако се нуждаете само от TypeScript типове (без код за изпълнение), използвайте стандартния импорт:

```typescript
// Само типове (без зависимости по време на изпълнение - безопасно навсякъде)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Използване на отделни API класове

#### Браузърна среда

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Сървърна среда  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```