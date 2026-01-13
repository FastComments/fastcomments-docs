---
SDK надає три основні API-класи:

- **`DefaultApi`** - Захищені кінцеві точки, які вимагають вашого API-ключа для автентифікації. Використовуйте їх для серверних операцій.
- **`PublicApi`** - Публічні кінцеві точки, до яких можна отримати доступ без API-ключа. Їх можна викликати безпосередньо з браузерів/мобільних пристроїв тощо.
- **`HiddenApi`** - Внутрішні/адміністративні кінцеві точки для просунутих сценаріїв використання.

### Приклад: Використання Public API (безпечне для браузера)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Отримати коментарі для сторінки (API-ключ не потрібен)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Приклад: Використання Default API (лише на сервері)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Тримайте це в секреті!
});
const defaultApi = new DefaultApi(config);

// Отримати коментарі з повним адміністративним доступом
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```
---