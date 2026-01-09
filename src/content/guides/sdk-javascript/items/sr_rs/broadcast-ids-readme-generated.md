Videćete da treba da prosledite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da ćete znati da ignorišete događaj ako planirate optimistično primeniti promene na klijentu (što verovatno želite, jer pruža najbolje iskustvo). Ovde prosledite UUID. ID treba biti dovoljno jedinstven da se ne pojavi dva puta tokom sesije pregledača.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Jedinstveni ID za ovu operaciju
  }
});
```