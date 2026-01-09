Le SDK fournit trois classes d'API principales :

- **`DefaultApi`** - Points de terminaison sécurisés qui nécessitent votre clé API pour l'authentification. Utilisez-les pour les opérations côté serveur.
- **`PublicApi`** - Points de terminaison publics accessibles sans clé API. Ils peuvent être appelés directement depuis les navigateurs/appareils mobiles/etc.
- **`HiddenApi`** - Points de terminaison internes/admin destinés aux cas d'utilisation avancés.

### Exemple : Utilisation de Public API (adaptée au navigateur)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Récupérer les commentaires d'une page (aucune clé API requise)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemple : Utilisation de Default API (uniquement côté serveur)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Gardez cette clé secrète !
});
const defaultApi = new DefaultApi(config);

// Récupérer les commentaires avec un accès administrateur complet
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```