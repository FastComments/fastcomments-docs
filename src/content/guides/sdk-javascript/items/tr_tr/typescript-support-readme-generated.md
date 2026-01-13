SDK, TypeScript ile yazılmıştır ve tüm API yöntemleri ile yanıt modelleri için eksiksiz tür tanımları sağlar:

```typescript
// Türleri varsayılan dışa aktarımdan içe aktar (her yerde güvenlidir)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Tarayıcı SDK'sı ile kullanın
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```