The SDK пружа ове API класе:

- **`DefaultApi`** - Заштићени крајњи пунктови који захтијевају ваш API key за аутентификацију. Користите их за операције на серверској страни.
- **`PublicApi`** - Јавни крајњи пунктови којима се може приступити без API key. Ове се могу позвати директно из прегледача/мобилних уређаја/итд.
- **`ModerationApi`** - Крајњи пунктови контролне плоче за модерацију (модерација коментара, забране, ознаке, фактор повјерења, претрага). Аутентификује се сесијом модератора; прослиједите параметар упита `sso` за модераторе аутентификоване преко SSO.
- **`HiddenApi`** - Интерни/админски крајњи пунктови за напредне случајеве употребе.

### Пример: Користећи Public API (сигурно за прегледник)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Добијање коментара за страницу (није потребан API key)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Користећи Default API (само на серверској страни)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Чувајте ово у тајности!
});
const defaultApi = new DefaultApi(config);

// Добијање коментара са пуним администраторским приступом
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Користећи Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, итд. */ });

// Позиви аутентификовани модератором (сесијски колачић, или прослиједите `sso` за модератора аутентификованог преко SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```