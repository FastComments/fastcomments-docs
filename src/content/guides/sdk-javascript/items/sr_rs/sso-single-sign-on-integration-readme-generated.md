FastComments подржава SSO за интеграцију са вашим постојећим системом аутентификације корисника. **SSO функционалност је доступна само у серверском експорту** јер захтева Node.js crypto могућности.

### Једноставан SSO (само серверска страна)

Једноставан SSO треба да се генерише на серверу и пошаље клијенту:

```typescript
// Код на серверу (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Направите једноставан SSO користећи уграђеног помоћника  
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

// Пошаљите ssoToken у код на клијентској страни
// Код на клијентској страни може затим да користи овај токен са SDK-ом за прегледач
```

### Сигуран SSO (серверска страна, препоручено)

Сигуран SSO треба да се имплементира на серверу и пружа бољу безбедност:

```typescript
// Код на серверу (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Направите сигуран SSO користећи уграђеног помоћника
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

// Користите са API позивима на серверу
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Или пошаљите ssoConfig клијенту за употребу у прегледачу
```

### Коришћење SSO у прегледачу (са сервером-генерисаним токеном)

```typescript
// Код на клијентској страни (прегледач)
import { PublicApi } from 'fastcomments-sdk/browser';

// Преузмите SSO токен са вашег серверског ендпоинта
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Користите SSO токен генерисан на серверу
});
```

### SSO са креирањем коментара

```typescript
// Серверска страна: креирајте SSO и коментар
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