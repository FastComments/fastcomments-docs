FastComments unterstützt SSO, um sich in Ihr bestehendes Benutzerauthentifizierungssystem zu integrieren. **Die SSO-Funktionalität ist nur im Server-Export verfügbar**, da sie Node.js-Crypto-Funktionen benötigt.

### Einfache SSO (nur serverseitig)

Einfache SSO sollte serverseitig erzeugt und an den Client gesendet werden:

```typescript
// Server-seitiger Code (Node.js/Backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Erstelle einfache SSO mit dem eingebauten Helfer  
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

// Sende ssoToken an Ihren Client-Code
// Der Client-Code kann dieses Token dann mit dem Browser-SDK verwenden
```

### Sichere SSO (serverseitig, empfohlen)

Sichere SSO sollte serverseitig implementiert werden und bietet bessere Sicherheit:

```typescript
// Server-seitiger Code (Node.js/Backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Erstelle sichere SSO mit dem eingebauten Helfer
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

// Mit API-Aufrufen auf dem Server verwenden
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Oder sende ssoConfig an den Client zur Browser-Verwendung
```

### Verwendung von SSO im Browser (mit servergeneriertem Token)

```typescript
// Client-seitiger Code (Browser)
import { PublicApi } from 'fastcomments-sdk/browser';

// Hole SSO-Token von Ihrem Server-Endpunkt
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO mit Kommentar-Erstellung

```typescript
// Server-seitig: SSO und Kommentar erstellen
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