---
いくつかの API 呼び出しでは `broadcastId` を渡す必要があることがわかります。イベントを受信するとこの ID が返ってくるので、クライアント側で楽観的に変更を適用することを計画している場合（これにより最良の体験が得られるため、通常はそうしたいでしょう）、そのイベントを無視すべきことが分かります。ここでは UUID を渡してください。この ID はブラウザセッション内で二度と発生しないほど十分にユニークである必要があります。

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // この操作のためのユニークID
  }
});
```
---