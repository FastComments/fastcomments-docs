SDK пружа три главне API класе:

- **`DefaultApi`** - Заштићени ендпоинти који захтијевају ваш API кључ за аутентификацију. Користите их за операције на серверској страни.
- **`PublicApi`** - Јавни ендпоинти којима се може приступити без API кључа. Могу се позивати директно из прегледача/мобилних уређаја/итд.
- **`HiddenApi`** - Интерни/админ ендпоинти за напредне случајеве употребе.

### Примјер: Коришћење Public API (погодно за прегледаче)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Добија коментаре за страницу (не захтијева се API кључ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Примјер: Коришћење Default API (само на серверској страни)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Чувајте ово у тајности!
});
const defaultApi = new DefaultApi(config);

// Добија коментаре са потпуним администраторским приступом
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```