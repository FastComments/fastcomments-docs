---
您会看到在某些 API 调用中需要传递 `broadcastId`。当您接收事件时，会拿回这个 ID，这样如果您计划在客户端乐观地应用更改（您可能会想这样做，因为它提供了最佳体验），就可以知道是否忽略该事件。在这里传入一个 UUID。该 ID 应足够唯一，以避免在同一浏览器会话中出现两次。

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // 此操作的唯一 ID
  }
});
```
---