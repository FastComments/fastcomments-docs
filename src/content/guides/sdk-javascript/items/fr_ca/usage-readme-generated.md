Ce SDK fournit des points d'entrée séparés pour les environnements navigateur et serveur afin d'assurer une compatibilité et une sécurité optimales :

### Utilisation dans le navigateur (côté client)

Pour les applications côté navigateur/front-end, utilisez l'export sécurisé pour le navigateur qui exclut les dépendances Node.js :

```typescript
// Import sécurisé pour le navigateur (aucune dépendance Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Crée une instance du SDK pour le navigateur
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optionnel, utilise https://fastcomments.com par défaut
});

// Utilise des API publiques (aucune clé API requise - sûr pour les navigateurs)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Utilisation côté serveur (Node.js)

Pour les applications côté serveur/back-end, utilisez le SDK complet avec les fonctionnalités SSO et d'authentification :

```typescript
// Import côté serveur (inclut le SSO et est conçu pour fonctionner avec NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Crée une instance du SDK côté serveur
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Gardez ceci secret sur le serveur !
  basePath: 'https://fastcomments.com' // optionnel, utilise https://fastcomments.com par défaut
});

// Utilise les API sécurisées avec votre clé API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Importation des types uniquement

Si vous avez seulement besoin des types TypeScript (aucun code à l'exécution), utilisez l'importation par défaut :

```typescript
// Uniquement les types (aucune dépendance à l'exécution - sûr partout)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Utilisation des classes API individuelles

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