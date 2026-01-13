FastComments supporta SSO per integrarsi con il tuo sistema di autenticazione utenti esistente. **La funzionalità SSO è disponibile solo nell'export lato server** poiché richiede le funzionalità crypto di Node.js.

### SSO semplice (solo lato server)

L'SSO semplice dovrebbe essere generato lato server e inviato al client:

```typescript
// Codice lato server (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crea SSO semplice usando l'aiuto integrato  
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

// Invia ssoToken al codice lato client
// Il codice lato client può quindi usare questo token con lo SDK per browser
```

### SSO sicuro (lato server, raccomandato)

L'SSO sicuro dovrebbe essere implementato lato server e offre maggiore sicurezza:

```typescript
// Codice lato server (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crea SSO sicuro usando l'aiuto integrato
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

// Usalo con chiamate API sul server
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Oppure invia ssoConfig al client per l'utilizzo nel browser
```

### Utilizzo dell'SSO dal browser (con token generato dal server)

```typescript
// Codice lato client (browser)
import { PublicApi } from 'fastcomments-sdk/browser';

// Ottieni il token SSO dal tuo endpoint server
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO con creazione di commenti

```typescript
// Lato server: crea SSO e commento
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