FastComments підтримує SSO для інтеграції з вашою існуючою системою аутентифікації користувачів. **Функціональність SSO доступна лише у серверному експорті**, оскільки вона вимагає можливостей Node.js crypto.

### Просте SSO (тільки на сервері)

Прості SSO-токени мають генеруватися на сервері та передаватися клієнту:

```typescript
// Серверний код (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Створити просте SSO за допомогою вбудованого помічника  
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

// Відправте ssoToken у ваш клієнтський код
// Клієнтський код потім може використати цей токен з браузерним SDK
```

### Безпечне SSO (серверна сторона, рекомендовано)

Безпечне SSO слід реалізовувати на сервері — воно забезпечує кращий рівень безпеки:

```typescript
// Серверний код (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Створити безпечне SSO за допомогою вбудованого помічника
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

// Використання з викликами API на сервері
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Або відправте ssoConfig клієнту для використання в браузері
```

### Використання SSO з браузера (з токеном, згенерованим на сервері)

```typescript
// Клієнтський код (браузер)
import { PublicApi } from 'fastcomments-sdk/browser';

// Отримати SSO токен з вашого серверного endpoint
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Використайте SSO токен, згенерований на сервері
});
```

### SSO зі створенням коментаря

```typescript
// На сервері: створити SSO і коментар
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