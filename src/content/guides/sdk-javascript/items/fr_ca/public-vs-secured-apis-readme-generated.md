Le SDK fournit ces classes d'API :

- **`DefaultApi`** - Points de terminaison sécurisés qui nécessitent votre clé API pour l'authentification. Utilisez-les pour les opérations côté serveur.
- **`PublicApi`** - Points de terminaison publics qui peuvent être accédés sans clé API. Ils peuvent être appelés directement depuis des navigateurs/appareils mobiles/etc.
- **`ModerationApi`** - Points de terminaison du tableau de bord de modération (modération des commentaires, bannissements, badges, facteur de confiance, recherche). Authentifié par la session du modérateur ; transmettez le paramètre de requête `sso` pour les modérateurs authentifiés par SSO.
- **`HiddenApi`** - Points de terminaison internes/admin pour des cas d'utilisation avancés.

### Exemple : Utiliser Public API (sécuritaire pour le navigateur)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Obtenir les commentaires d'une page (aucune clé API requise)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemple : Utiliser Default API (côté serveur uniquement)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Gardez ceci secret!
});
const defaultApi = new DefaultApi(config);

// Obtenir les commentaires avec un accès administrateur complet
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemple : Utiliser Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Appels authentifiés par le modérateur (cookie de session, ou passer `sso` pour un modérateur authentifié via SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```