SDK je napisan v TypeScriptu in zagotavlja popolne definicije tipov za vse API metode in modele odgovorov:

```typescript
// Uvozi tipe iz privzetega izvoza (varno povsod)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Uporabi z brskalniškim SDK
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```