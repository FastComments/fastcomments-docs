---
Vidjet ćete da trebate proslijediti `broadcastId` u nekim API pozivima. Kada primite događaje, dobit ćete ovaj ID natrag, tako da znate zanemariti događaj ako planirate optimistično primijeniti promjene na klijentu (što ćete vjerojatno htjeti učiniti jer pruža najbolji doživljaj). Ovdje proslijedite UUID. ID bi trebao biti dovoljno jedinstven da se ne pojavi dva puta u istoj sesiji preglednika.

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
---