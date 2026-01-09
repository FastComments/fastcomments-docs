你會看到在某些 API 呼叫中應該傳入 `broadcastId`。當你接收事件時，你會得到這個 ID 回傳，因此如果你打算在客戶端採取樂觀更新（你很可能會想這麼做，因為它提供最佳體驗），你就會知道要忽略該事件。請在此傳入 UUID。該 ID 應該足夠唯一，以免在瀏覽器會話中出現兩次。

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // 此操作的唯一識別碼
  }
});
```