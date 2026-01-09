Ce SDK fournit des points d'entrée distincts pour les environnements navigateur et serveur afin d'assurer une compatibilité et une sécurité optimales :

### Utilisation dans le navigateur (côté client)

Pour les applications front-end/navigateurs, utilisez l'export sécurisé pour le navigateur qui exclut les dépendances Node.js :

```typescript
// Import sécurisé pour le navigateur (sans dépendances Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Créer une instance du SDK pour le navigateur
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optionnel, valeur par défaut : https://fastcomments.com
});

// Utiliser les API publiques (pas de clé API nécessaire - sûr pour les navigateurs)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Utilisation côté serveur (Node.js)

Pour les applications serveur/back-end, utilisez le SDK complet avec les fonctionnalités SSO et d'authentification :

```typescript
// Import côté serveur (inclut SSO et conçu pour fonctionner avec NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Créer une instance du SDK pour le serveur
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Gardez ceci secret sur le serveur !
  basePath: 'https://fastcomments.com' // optionnel, valeur par défaut : https://fastcomments.com
});

// Utiliser les API sécurisées avec votre clé API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Import de types uniquement

Si vous avez seulement besoin des types TypeScript (aucun code d'exécution), utilisez l'import par défaut :

```typescript
// Types uniquement (aucune dépendance d'exécution - sûr partout)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Utilisation des classes d'API individuelles

#### Environnement navigateur

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Environnement serveur  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```