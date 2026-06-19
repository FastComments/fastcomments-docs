SDK предоставляет следующие классы API:

- **`DefaultApi`** - Защищенные конечные точки, для доступа к которым требуется ваш API-ключ. Используйте их для серверных операций.
- **`PublicApi`** - Публичные конечные точки, доступ к которым возможен без API-ключа. Их можно вызывать напрямую из браузеров/мобильных приложений и т.д.
- **`ModerationApi`** - Конечные точки панели модератора (модерация комментариев, баны, бейджи, trust factor, поиск). Аутентифицируются сессией модератора; передайте параметр запроса `sso` для модераторов с SSO-аутентификацией.
- **`HiddenApi`** - Внутренние/админские конечные точки для продвинутых сценариев использования.

### Example: Using Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Получить комментарии для страницы (API-ключ не требуется)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Храните это в секрете!
});
const defaultApi = new DefaultApi(config);

// Получить комментарии с полным доступом администратора
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Example: Using Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath и т.д. */ });

// Вызовы, аутентифицированные как модератор (куки сессии или передайте `sso` для SSO-модератора).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```