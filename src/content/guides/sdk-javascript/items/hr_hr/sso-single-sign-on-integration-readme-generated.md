FastComments podržava SSO za integraciju s vašim postojećim sustavom autentikacije korisnika. **SSO funkcionalnost dostupna je samo u server exportu** jer zahtijeva kriptografske značajke Node.js.

### Jednostavni SSO (samo na strani poslužitelja)

Jednostavni SSO treba generirati na strani poslužitelja i poslati klijentu:

```typescript
// Kôd na strani poslužitelja (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Kreirajte jednostavni SSO koristeći ugrađeni helper  
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

// Pošaljite ssoToken vašem klijentskom kodu
// Klijentski kod može tada koristiti ovaj token s browser SDK-om
```

### Sigurni SSO (na strani poslužitelja, preporučeno)

Sigurni SSO treba implementirati na strani poslužitelja i pruža bolju sigurnost:

```typescript
// Kôd na strani poslužitelja (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Kreirajte siguran SSO koristeći ugrađeni helper
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

// Koristite s API pozivima na poslužitelju
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ili pošaljite ssoConfig klijentu za korištenje u pregledniku
```

### Korištenje SSO iz preglednika (s tokenom generiranim na serveru)

```typescript
// Klijentski kod (preglednik)
import { PublicApi } from 'fastcomments-sdk/browser';

// Dohvatite SSO token s vašeg server endpointa
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Koristite token SSO generiran na serveru
});
```

### SSO s kreiranjem komentara

```typescript
// Na strani poslužitelja: Kreirajte SSO i komentar
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