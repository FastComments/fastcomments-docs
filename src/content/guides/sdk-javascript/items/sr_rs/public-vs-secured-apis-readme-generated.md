SDK пружа три главне API класе:

- **`DefaultApi`** - Заштићени крајњи пункти који захтевају ваш API кључ за аутентификацију. Користите их за операције на серверској страни.
- **`PublicApi`** - Јавни крајњи пункти који су доступни без API кључа. Могу се позивати директно из прегледача/мобилних уређаја/итд.
- **`HiddenApi`** - Интерни/админ крајњи пункти за напредне случајеве употребе.

### Пример: Коришћење Public API (безбедно за прегледач)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Добијање коментара за страницу (API кључ није потребан)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Коришћење Default API (само на серверу)

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