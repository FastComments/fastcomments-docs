SDK je napisan u TypeScriptu i pruža potpune definicije tipova za sve API metode i modele odgovora:

```typescript
// Uvozi tipove iz zadanog izvoza (sigurno svugdje)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Koristite s SDK-om za preglednik
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```