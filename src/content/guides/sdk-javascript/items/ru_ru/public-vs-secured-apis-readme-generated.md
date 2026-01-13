SDK предоставляет три основных класса API:

- **`DefaultApi`** - Защищённые конечные точки, для которых требуется ваш API-ключ для аутентификации. Используйте их для операций на стороне сервера.
- **`PublicApi`** - Публичные конечные точки, к которым можно получить доступ без API-ключа. Их можно вызывать напрямую из браузеров/мобильных устройств и т.д.
- **`HiddenApi`** - Внутренние/административные конечные точки для продвинутых сценариев использования.

### Пример: Использование Public API (безопасно для браузера)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Получить комментарии для страницы (API-ключ не требуется)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Использование Default API (только на стороне сервера)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Держите это в секрете!
});
const defaultApi = new DefaultApi(config);

// Получить комментарии с полным доступом администратора
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```