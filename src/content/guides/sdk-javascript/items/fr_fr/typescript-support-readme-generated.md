---
Le SDK est écrit en TypeScript et fournit des définitions de types complètes pour toutes les méthodes de l'API et les modèles de réponse:

```typescript
// Importer des types depuis l'exportation par défaut (compatible partout)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// À utiliser avec le SDK pour le navigateur
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```
---