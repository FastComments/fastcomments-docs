---
SDK е написан на TypeScript и предоставя пълни типови дефиниции за всички API методи и модели за отговори:

```typescript
// Импортиране на типове от подразбиращ се експорт (безопасно навсякъде)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// За използване с браузър SDK
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```
---