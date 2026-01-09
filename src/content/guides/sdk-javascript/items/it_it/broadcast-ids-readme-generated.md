Vedrai che dovrai passare un `broadcastId` in alcune chiamate API. Quando riceverai eventi, ti verrà restituito questo ID, quindi saprai di ignorare l'evento se intendi applicare le modifiche in modo ottimistico sul client (cosa che probabilmente vorrai fare, dato che offre la migliore esperienza). Passa qui un UUID. L'ID dovrebbe essere sufficientemente unico da non verificarsi due volte durante la sessione del browser.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // ID univoco per questa operazione
  }
});
```