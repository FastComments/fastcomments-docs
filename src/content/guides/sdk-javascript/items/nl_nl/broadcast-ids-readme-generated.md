Je ziet dat je in sommige API-aanroepen een `broadcastId` moet doorgeven. Wanneer je events ontvangt, krijg je deze ID terug, zodat je het event kunt negeren als je van plan bent om wijzigingen optimistisch op de client toe te passen (wat je waarschijnlijk wilt doen omdat het de beste ervaring biedt). Geef hier een UUID door. De ID moet uniek genoeg zijn om niet twee keer in een browsersessie voor te komen.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Unieke ID voor deze bewerking
  }
});
```