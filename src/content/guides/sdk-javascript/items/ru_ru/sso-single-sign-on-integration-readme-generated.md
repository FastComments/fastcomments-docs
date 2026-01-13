FastComments поддерживает SSO для интеграции с вашей существующей системой аутентификации пользователей. **Функциональность SSO доступна только в серверном экспорте**, так как требует возможностей crypto в Node.js.

### Simple SSO (Server-Side Only)

Простой SSO должен генерироваться на стороне сервера и отправляться клиенту:

```typescript
// Серверный код (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create simple SSO using the built-in helper  
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

// Отправьте ssoToken на клиент
// Код на стороне клиента затем может использовать этот токен с браузерным SDK
```

### Secure SSO (Server-Side, Recommended)

Защищённый SSO должен реализовываться на стороне сервера и обеспечивает лучшую безопасность:

```typescript
// Серверный код (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Create secure SSO using the built-in helper
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

// Используйте при вызовах API на сервере
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Или отправьте ssoConfig клиенту для использования в браузере
```

### Using SSO from Browser (with Server-Generated Token)

```typescript
// Клиентский код (браузер)
import { PublicApi } from 'fastcomments-sdk/browser';

// Получите SSO токен с вашего серверного эндпоинта
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO with Comment Creation

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