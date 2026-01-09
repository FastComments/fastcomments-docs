Sie werden sehen, dass Sie in einigen API-Aufrufen eine `broadcastId` übergeben sollen. Wenn Sie Ereignisse erhalten, bekommen Sie diese ID zurück, sodass Sie das Ereignis ignorieren können, falls Sie Änderungen auf dem Client optimistisch anwenden möchten (was Sie wahrscheinlich tun sollten, da es die beste Erfahrung bietet). Geben Sie hier eine UUID an. Die ID sollte so eindeutig sein, dass sie in einer Browsersitzung nicht zweimal vorkommt.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Eindeutige ID für diese Operation
  }
});
```