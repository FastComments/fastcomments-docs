---
Vous verrez qu'il faut transmettre un `broadcastId` dans certains appels d'API. Quand vous recevez des événements, vous récupérerez cet ID, ce qui vous permet d'ignorer l'événement si vous comptez appliquer les changements de manière optimiste côté client (ce que vous voudrez probablement faire car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois durant une session de navigateur.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // ID unique pour cette opération
  }
});
```
---