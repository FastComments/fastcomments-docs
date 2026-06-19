SDK обезбеђује следеће API класе:

- **`DefaultApi`** - Заштићени ендпоинти који захтевају ваш API кључ за аутентификацију. Користите их за серверске операције.
- **`PublicApi`** - Јавни ендпоинти који се могу приступити без API кључа. Могу се позивати директно из претраживача/мобилних уређаја/итд.
- **`ModerationApi`** - Ендпоинти контролне табле модератора (модерација коментара, забране, значке, фактор поверења, претрага). Аутентикује се помоћу сесије модератора; проследите query параметар `sso` за модераторе аутентификоване преко SSO.
- **`HiddenApi`** - Интерни/админ ендпоинти за напредне случајеве употребе.

### Пример: Коришћење Public API-ја (безбедно за претраживач)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Добијте коментаре за страницу (није потребан API кључ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Коришћење Default API-ја (само на серверу)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Чувајте ово у тајности!
});
const defaultApi = new DefaultApi(config);

// Добијање коментара са пуном админ приступом
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Коришћење Moderation API-ја

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, итд. */ });

// Позиви аутентификованог модератора (колачић сесије, или проследите `sso` за SSO модератора).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```