FastComments поддерживает SSO для интеграции с вашей существующей системой аутентификации пользователей. **Функциональность SSO доступна только в серверном экспорте**, поскольку она требует возможностей crypto в Node.js.

### Простой SSO (только на стороне сервера)

Простой SSO должен генерироваться на стороне сервера и отправляться клиенту:

```typescript
// Код на стороне сервера (Node.js/бэкенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Создать простой SSO с помощью встроенного помощника  
const userData = {
  username: 'john_doe',
  email: 'john@example.com',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg'
};

const sso = FastCommentsSSO.createSimple(userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoToken = sso.createToken();

// Отправьте ssoToken в код на стороне клиента
// Код на стороне клиента затем может использовать этот токен с браузерным SDK
```

### Защищённый SSO (на стороне сервера, рекомендуется)

Защищённый SSO должен реализовываться на стороне сервера и обеспечивает лучшую безопасность:

```typescript
// Код на стороне сервера (Node.js/бэкенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Создать защищённый SSO с помощью встроенного помощника
const userData = {
  id: 'user-123',
  email: 'john@example.com',
  username: 'john_doe',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg',
  isAdmin: false,
  isModerator: false
};

const sso = FastCommentsSSO.createSecure('your-api-key', userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoConfig = sso.prepareToSend();

// Использовать с вызовами API на сервере
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Или отправить ssoConfig клиенту для использования в браузере
```

### Использование SSO из браузера (с серверно-сгенерированным токеном)

```typescript
// Код на стороне клиента (браузер)
import { PublicApi } from 'fastcomments-sdk/browser';

// Получите SSO-токен с вашего серверного эндпоинта
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Использовать сгенерированный сервером SSO-токен
});
```

### SSO при создании комментария

```typescript
// На стороне сервера: создать SSO и комментарий
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

const sso = FastCommentsSSO.createSecure('your-api-key', userData);
const ssoConfig = sso.prepareToSend();

const response = await publicApi.createCommentPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  broadcastId: 'unique-broadcast-id',
  commentData: {
    comment: 'This is my comment',
    date: Date.now(),
    commenterName: 'John Doe',
    url: 'https://example.com/page',
    urlId: 'page-url-id'
  },
  sso: JSON.stringify(ssoConfig)
});
```