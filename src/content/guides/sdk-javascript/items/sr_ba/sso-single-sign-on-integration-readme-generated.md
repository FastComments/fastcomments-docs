FastComments подржава SSO за интеграцију са вашим постојећим системом аутентификације корисника. **SSO функционалност је доступна само у серверском експорту** јер захтијева Node.js crypto функције.

### Једноставан SSO (само на страни сервера)

Једноставан SSO треба да буде генерисан на страни сервера и послат клијенту:

```typescript
// Код на страни сервера (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Креирајте једноставан SSO користећи уграђени помоћник  
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

// Пошаљите ssoToken вашем коду на клијенту
// Код на клијенту затим може користити овај токен са browser SDK-ом
```

### Сигуран SSO (на страни сервера, препоручено)

Сигуран SSO треба имплементирати на страни сервера и обезбјеђује бољу безбједност:

```typescript
// Код на страни сервера (Node.js/бекенд)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Креирајте сигуран SSO користећи уграђени помоћник
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

// Користити са API позивима на серверу
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Или пошаљите ssoConfig клијенту за употребу у прегледачу
```

### Коришћење SSO из прегледача (са токеном генерисаним на серверу)

```typescript
// Код на клијенту (прегледач)
import { PublicApi } from 'fastcomments-sdk/browser';

// Добијте SSO токен са вашег серверског endpoint-а
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
// На страни сервера: Креирајте SSO и коментар
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