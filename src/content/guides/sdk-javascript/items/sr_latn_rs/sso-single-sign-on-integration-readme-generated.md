FastComments podržava SSO za integraciju sa vašim postojećim sistemom autentifikacije korisnika. **SSO funkcionalnost je dostupna samo u server export verziji** jer zahteva Node.js crypto funkcije.

### Jednostavno SSO (samo na serveru)

Jednostavno SSO treba da se generiše na serveru i pošalje klijentu:

```typescript
// Kod na strani servera (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Kreiraj jednostavno SSO koristeći ugrađeni helper  
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

// Pošaljite ssoToken svom kodu na strani klijenta
// Kod na strani klijenta potom može da koristi ovaj token sa browser SDK-om
```

### Sigurno SSO (na strani servera, preporučeno)

Sigurno SSO treba da se implementira na serveru i pruža bolju bezbednost:

```typescript
// Kod na strani servera (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Kreiraj sigurno SSO koristeći ugrađeni helper
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

// Koristi sa API pozivima na serveru
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ili pošaljite ssoConfig klijentu za korišćenje u pregledaču
```

### Korišćenje SSO iz pregledača (sa tokenom generisanim na serveru)

```typescript
// Kod na strani klijenta (pregledač)
import { PublicApi } from 'fastcomments-sdk/browser';

// Preuzmi SSO token sa svog server endpoint-a
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Koristi SSO token generisan na serveru
});
```

### SSO sa kreiranjem komentara

```typescript
// Na strani servera: kreiraj SSO i komentar
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