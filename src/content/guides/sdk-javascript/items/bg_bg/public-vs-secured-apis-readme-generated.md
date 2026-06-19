SDK предоставя тези API класове:

- **`DefaultApi`** - Защитени крайни точки, които изискват вашия API ключ за удостоверяване. Използвайте ги за операции от страна на сървъра.
- **`PublicApi`** - Публични крайни точки, които могат да се използват без API ключ. Могат да бъдат извиквани директно от браузъри/мобилни устройства/и т.н.
- **`ModerationApi`** - Крайни точки за таблото на модератора (модериране на коментари, забрани, значки, фактор на доверие, търсене). Удостоверяват се чрез сесията на модератора; предайте query параметъра `sso` за SSO-удостоверени модератори.
- **`HiddenApi`** - Вътрешни/административни крайни точки за напреднали случаи на използване.

### Пример: Използване на Public API (безопасно за браузър)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Вземете коментари за страница (не е необходим API ключ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Използване на Default API (само от страна на сървъра)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Запазете това в тайна!
});
const defaultApi = new DefaultApi(config);

// Get comments with full admin access
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Използване на Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, и т.н. */ });

// Moderator-authenticated calls (session cookie, or pass `sso` for an SSO moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```