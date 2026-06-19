SDK пружа ове API класе:

- **`DefaultApi`** - Заштићене крајње тачке које захтијевају ваш API кључ за аутентификацију. Користите их за серверске операције.
- **`PublicApi`** - Јавне крајње тачке којима се може приступити без API кључа. Ове се могу позивати директно из прегледача/мобилних уређаја/итд.
- **`ModerationApi`** - Крајње тачке контролне табле за модерацију (модерација коментара, забране, значке, фактор повјерења, претрага). Аутентификовано сесијом модератора; прослиједите query параметар `sso` за модераторе аутентификоване преко SSO.
- **`HiddenApi`** - Интерне/администраторске крајње тачке за напредне случајеве употребе.

### Примјер: Коришћење јавног API-ја (безбједно за прегледач)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Преузимање коментара за страницу (API кључ није потребан)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Примјер: Коришћење Default API-а (само на серверу)

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

### Примјер: Коришћење API-ја за модерацију

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, итд. */ });

// Позиви аутентификовани као модератор (сесијски колачић, или прослиједите `sso` за SSO модератора).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```