Вы увидите, что в некоторых вызовах API требуется передать `broadcastId`. Когда вы получаете события, вы получите этот ID обратно, поэтому сможете игнорировать событие, если планируете оптимистично применять изменения на клиенте (а вам, вероятно, захочется так сделать, поскольку это обеспечивает лучший опыт). Передавайте здесь UUID. ID должен быть достаточно уникальным, чтобы не повторяться в рамках одной сессии браузера.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Уникальный идентификатор для этой операции
  }
});
```