SDK надає наступні класи API:

- **`DefaultApi`** - Захищені кінцеві точки, які вимагають вашого API-ключа для автентифікації. Використовуйте їх для серверних операцій.
- **`PublicApi`** - Публічні кінцеві точки, до яких можна отримати доступ без API-ключа. Їх можна викликати безпосередньо з браузерів/мобільних пристроїв тощо.
- **`ModerationApi`** - Кінцеві точки панелі модератора (модерація коментарів, блокування, бейджі, фактор довіри, пошук). Автентифікація через сесію модератора; передайте параметр запиту `sso` для модераторів, автентифікованих через SSO.
- **`HiddenApi`** - Внутрішні/адміністративні кінцеві точки для розширених випадків використання.

### Приклад: Використання Public API (browser-safe)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Отримати коментарі для сторінки (API-ключ не потрібен)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Приклад: Використання Default API (server-side only)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Тримайте це в секреті!
});
const defaultApi = new DefaultApi(config);

// Отримати коментарі з повним доступом адміністратора
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Приклад: Використання Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Виклики, автентифіковані модератором (cookie сесії, або передайте `sso` для модератора з SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```