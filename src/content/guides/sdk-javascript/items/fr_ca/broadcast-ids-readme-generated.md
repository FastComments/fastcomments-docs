Vous verrez que vous devez transmettre un `broadcastId` dans certains appels d'API. Lorsque vous recevrez des événements, vous recevrez cet ID en retour, ce qui vous permet d'ignorer l'événement si vous prévoyez d'appliquer les modifications de façon optimiste sur le client (ce que vous voudrez probablement faire, car cela offre la meilleure expérience). Fournissez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois lors d'une session de navigateur.

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