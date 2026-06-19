SDK предоставляет следующие классы API:

- **`DefaultApi`** - Защищенные конечные точки, требующие ваш API-ключ для аутентификации. Используйте их для серверных операций.
- **`PublicApi`** - Публичные конечные точки, к которым можно получить доступ без API-ключа. Их можно вызывать напрямую из браузеров/мобильных устройств и т.д.
- **`ModerationApi`** - Конечные точки панели модератора (модерация комментариев, баны, значки, коэффициент доверия, поиск). Аутентификация через сессию модератора; передайте параметр запроса `sso` для модераторов, аутентифицированных через SSO.
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

### Пример: Использование Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath и т. д. */ });

// Вызовы, аутентифицированные модератором (сессионное cookie, или передайте `sso` для модератора, аутентифицированного через SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```