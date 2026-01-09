FastComments podpira SSO za integracijo z vašim obstoječim sistemom za preverjanje pristnosti uporabnikov. **Funkcionalnost SSO je na voljo le v strežniškem izvozu** saj zahteva Node.js kriptografske funkcije.

### Preprosto SSO (samo strežniško)

Preprosto SSO je treba generirati na strežniku in poslati odjemalcu:

```typescript
// Strežniška koda (Node.js/strežnik)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Ustvari preprosto SSO z vgrajenim pripomočkom  
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

// Pošlji ssoToken vaši odjemalski kodi
// Odjemalska koda lahko nato uporabi ta žeton z brskalniškim SDK-jem
```

### Varen SSO (strežniško, priporočeno)

Varen SSO je treba implementirati na strežniku in nudi boljšo varnost:

```typescript
// Strežniška koda (Node.js/strežnik)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Ustvari varen SSO z vgrajenim pripomočkom
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

// Uporabi z API klici na strežniku
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ali pošlji ssoConfig odjemalcu za uporabo v brskalniku
```

### Uporaba SSO iz brskalnika (z žetonom, ustvarjenim na strežniku)

```typescript
// Odjemalska koda (brskalnik)
import { PublicApi } from 'fastcomments-sdk/browser';

// Pridobi SSO žeton iz vašega strežniškega končnega mesta
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Uporabi strežniško ustvarjen SSO žeton
});
```

### SSO z ustvarjanjem komentarja

```typescript
// Strežniško: ustvari SSO in komentar
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