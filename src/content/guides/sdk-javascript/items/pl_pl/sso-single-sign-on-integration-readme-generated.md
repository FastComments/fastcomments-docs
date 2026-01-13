FastComments obsługuje SSO w celu integracji z istniejącym systemem uwierzytelniania użytkowników. **Funkcjonalność SSO jest dostępna tylko w eksporcie serwera** ponieważ wymaga funkcji kryptograficznych Node.js.

### Proste SSO (tylko po stronie serwera)

Proste SSO powinno być generowane po stronie serwera i wysyłane do klienta:

```typescript
// Kod po stronie serwera (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Utwórz proste SSO przy użyciu wbudowanego pomocnika  
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

// Wyślij ssoToken do kodu po stronie klienta
// Kod po stronie klienta może następnie użyć tego tokena z SDK przeglądarki
```

### Bezpieczne SSO (po stronie serwera, zalecane)

Bezpieczne SSO powinno być zaimplementowane po stronie serwera i zapewnia lepsze bezpieczeństwo:

```typescript
// Kod po stronie serwera (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Utwórz bezpieczne SSO przy użyciu wbudowanego pomocnika
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

// Użyj z wywołaniami API na serwerze
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Lub wyślij ssoConfig do klienta do użycia w przeglądarce
```

### Użycie SSO z przeglądarki (z tokenem wygenerowanym przez serwer)

```typescript
// Kod po stronie klienta (przeglądarka)
import { PublicApi } from 'fastcomments-sdk/browser';

// Pobierz token SSO z endpointu swojego serwera
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use the server-generated SSO token
});
```

### SSO z tworzeniem komentarza

```typescript
// Po stronie serwera: utwórz SSO i komentarz
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