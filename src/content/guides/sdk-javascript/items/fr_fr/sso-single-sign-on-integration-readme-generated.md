FastComments prend en charge le SSO pour s'intégrer à votre système d'authentification utilisateur existant. **La fonctionnalité SSO n'est disponible que dans l'export côté serveur** car elle nécessite les fonctionnalités crypto de Node.js.

### SSO simple (côté serveur uniquement)

Le SSO simple doit être généré côté serveur et envoyé au client :

```typescript
// Code côté serveur (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Créez un SSO simple en utilisant l'assistant intégré  
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

// Envoyez ssoToken à votre code côté client
// Le code côté client peut ensuite utiliser ce jeton avec le SDK pour navigateur
```

### SSO sécurisé (côté serveur, recommandé)

Le SSO sécurisé doit être implémenté côté serveur et offre une meilleure sécurité :

```typescript
// Code côté serveur (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Créez un SSO sécurisé en utilisant l'assistant intégré
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

// À utiliser avec les appels API côté serveur
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ou envoyez ssoConfig au client pour une utilisation dans le navigateur
```

### Utiliser le SSO depuis le navigateur (avec un jeton généré par le serveur)

```typescript
// Code côté client (navigateur)
import { PublicApi } from 'fastcomments-sdk/browser';

// Récupérez le jeton SSO depuis votre endpoint serveur
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Utilisez le jeton SSO généré par le serveur
});
```

### SSO avec création de commentaire

```typescript
// Côté serveur : créer le SSO et le commentaire
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