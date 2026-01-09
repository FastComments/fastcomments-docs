---
SDK предоставя три основни API класа:

- **`DefaultApi`** - Защитени крайни точки, които изискват вашия API ключ за удостоверяване. Използвайте тези за операции от страна на сървъра.
- **`PublicApi`** - Публични крайни точки, които могат да бъдат достъпвани без API ключ. Те могат да се извикват директно от браузъри/мобилни устройства/и т.н.
- **`HiddenApi`** - Вътрешни/административни крайни точки за напреднали случаи на използване.

### Пример: Използване на Public API (безопасно за браузър)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Получете коментари за страница (не е необходим API ключ)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Пример: Използване на Default API (само от страна на сървъра)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Пазете това в тайна!
});
const defaultApi = new DefaultApi(config);

// Получете коментари с пълен администраторски достъп
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```
---