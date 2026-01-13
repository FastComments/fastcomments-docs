Videćete da treba da prosledite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, tako da znate da zanemarite događaj ako planirate da optimistično primenite izmene na klijentu (što ćete verovatno želeti da uradite jer pruža najbolje iskustvo). Prosledite ovde UUID. ID bi trebalo da bude dovoljno jedinstven da se ne pojavi dva puta u okviru jedne sesije pregledača.

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