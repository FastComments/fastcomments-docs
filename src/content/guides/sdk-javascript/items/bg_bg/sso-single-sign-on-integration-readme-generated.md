FastComments поддържа SSO за интеграция с вашата съществуваща система за удостоверяване на потребители. **Функционалността SSO е достъпна само в сървърния експорт** тъй като изисква криптографски възможности на Node.js.

### Прост SSO (само от страна на сървъра)

Прост SSO трябва да се генерира от страна на сървъра и да се изпрати на клиента:

```typescript
// Серверен код (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Създаване на прост SSO с помощта на вградения помощник  
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

// Изпратете ssoToken към клиентския код
// Клиентският код може да използва този токен с browser SDK
```

### Сигурен SSO (от страна на сървъра, препоръчително)

Сигурният SSO трябва да се имплементира от страна на сървъра и предлага по-добра сигурност:

```typescript
// Серверен код (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Създаване на сигурен SSO с помощта на вградения помощник
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

// Използване с API повиквания на сървъра
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Или изпратете ssoConfig на клиента за използване в браузъра
```

### Използване на SSO от браузъра (с токен, генериран от сървъра)

```typescript
// Клиентски код (браузър)
import { PublicApi } from 'fastcomments-sdk/browser';

// Вземете SSO токен от вашия сървърен endpoint
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Използвайте SSO токена, генериран от сървъра
});
```

### SSO с създаване на коментар

```typescript
// От страна на сървъра: Създайте SSO и коментар
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