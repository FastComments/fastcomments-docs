일부 API 호출에서는 `broadcastId`를 전달해야 한다는 것을 보게 될 것입니다. 이벤트를 수신하면 이 ID가 반환되므로, 클라이언트에서 낙관적으로 변경을 적용하려는 경우(가장 좋은 경험을 제공하므로 아마도 그렇게 하게 될 것입니다) 해당 이벤트를 무시해야 할지 알 수 있습니다. 여기에는 UUID를 전달하세요. 이 ID는 브라우저 세션에서 두 번 발생하지 않을 정도로 충분히 고유해야 합니다.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // 이 작업을 위한 고유 ID
  }
});
```