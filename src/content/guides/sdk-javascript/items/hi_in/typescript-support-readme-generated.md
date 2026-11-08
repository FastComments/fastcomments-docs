SDK TypeScript में लिखा गया है और सभी API मेथड्स तथा रिस्पॉन्स मॉडल्स के लिए पूर्ण टाइप डेफिनिशन्स प्रदान करता है:

```typescript
// Import types from the default export (safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Use with browser SDK
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```