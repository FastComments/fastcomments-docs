Этот SDK предоставляет отдельные точки входа для браузерной и серверной сред, чтобы обеспечить оптимальную совместимость и безопасность:

### Использование в браузере (клиентская сторона)

Для браузерных/фронтенд-приложений используйте экспорт, безопасный для браузера, который исключает зависимости Node.js:

```typescript
// Импорт, безопасный для браузера (без зависимостей Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // необязательно, по умолчанию https://fastcomments.com
});

// Использовать публичные API (API-ключ не нужен - безопасно для браузера)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Использование на сервере (Node.js)

Для серверных/бэкенд-приложений используйте полный SDK с поддержкой SSO и функциями аутентификации:

```typescript
// Импорт для сервера (включает SSO и предназначен для работы с NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Храните это в секрете на сервере!
  basePath: 'https://fastcomments.com' // необязательно, по умолчанию https://fastcomments.com
});

// Использовать защищённые API с вашим API-ключом
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Только импорт типов

Если вам нужны только типы TypeScript (без кода выполнения), используйте импорт по умолчанию:

```typescript
// Только типы (нет зависимостей на этапе выполнения - безопасно везде)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Использование отдельных классов API

#### Браузерная среда

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Серверная среда  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```