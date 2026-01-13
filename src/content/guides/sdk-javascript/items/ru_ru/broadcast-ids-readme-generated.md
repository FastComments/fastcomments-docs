Вы увидите, что в некоторых вызовах API требуется передавать `broadcastId`. Когда вы получаете события, вы получите этот ID обратно, так что сможете игнорировать событие, если планируете оптимистично применять изменения на клиенте (и, вероятно, вам захочется так сделать, поскольку это обеспечивает лучший опыт). Передайте здесь UUID. Этот ID должен быть достаточно уникальным, чтобы не повториться дважды в рамках одной сессии браузера.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Уникальный ID для этой операции
  }
});
```