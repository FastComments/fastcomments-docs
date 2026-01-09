FastComments ondersteunt SSO om te integreren met uw bestaande gebruikersauthenticatiesysteem. **SSO-functionaliteit is alleen beschikbaar in de server-export** omdat het Node.js crypto-functies vereist.

### Eenvoudige SSO (Alleen serverzijde)

Eenvoudige SSO moet aan de serverzijde worden gegenereerd en naar de client worden gestuurd:

```typescript
// Code aan de serverzijde (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Maak eenvoudige SSO met de ingebouwde helper  
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

// Stuur ssoToken naar uw client-side code
// Client-side code kan dit token vervolgens gebruiken met de browser SDK
```

### Beveiligde SSO (Server-side, aanbevolen)

Beveiligde SSO moet aan de serverzijde worden geïmplementeerd en biedt betere beveiliging:

```typescript
// Code aan de serverzijde (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Maak beveiligde SSO met de ingebouwde helper
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

// Gebruik met API-aanroepen op de server
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Of stuur ssoConfig naar de client voor gebruik in de browser
```

### SSO gebruiken vanuit de browser (met door de server gegenereerd token)

```typescript
// Code aan de clientzijde (browser)
import { PublicApi } from 'fastcomments-sdk/browser';

// Haal SSO-token op van uw server-endpoint
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Gebruik het door de server gegenereerde SSO-token
});
```

### SSO met het aanmaken van een reactie

```typescript
// Server-side: Maak SSO en reactie
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