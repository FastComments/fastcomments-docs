Videćete da treba da prosledite `broadcastId` u nekim API pozivima. Kada primite događaje, dobićete ovaj ID nazad, pa ćete znati da ignorišete događaj ako planirate optimistično primeniti promene na klijentu (što ćete verovatno želeti da uradite, jer pruža najbolje iskustvo). Pošaljite ovde UUID. ID treba da bude dovoljno jedinstven da se ne pojavi dva puta tokom jedne sesije preglednika.

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