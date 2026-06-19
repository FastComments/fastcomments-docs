Le SDK fournit ces classes d'API :

- **`DefaultApi`** - Points de terminaison sécurisés qui nécessitent votre API key pour l'authentification. Utilisez-les pour les opérations côté serveur.
- **`PublicApi`** - Points de terminaison publics accessibles sans API key. Ils peuvent être appelés directement depuis des navigateurs, appareils mobiles, etc.
- **`ModerationApi`** - Points de terminaison du tableau de bord des modérateurs (modération des commentaires, bannissements, badges, facteur de confiance, recherche). Authentifié par la session du modérateur ; passez le paramètre de requête `sso` pour les modérateurs authentifiés via SSO.
- **`HiddenApi`** - Points de terminaison internes/admin pour des cas d'utilisation avancés.

### Exemple : Utilisation de l'API publique (compatible navigateur)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Récupère les commentaires d'une page (pas d'API key requise)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemple : Utilisation de l'API Default (côté serveur uniquement)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Gardez ceci secret!
});
const defaultApi = new DefaultApi(config);

// Récupère les commentaires avec un accès administrateur complet
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemple : Utilisation de l'API de modération

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Appels authentifiés par le modérateur (cookie de session, ou passez `sso` pour un modérateur SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```