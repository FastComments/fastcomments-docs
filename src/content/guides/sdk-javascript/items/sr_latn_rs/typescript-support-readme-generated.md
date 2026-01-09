SDK je napisan u TypeScript-u i pruža kompletne definicije tipova za sve API metode i modele odgovora:

```typescript
// Uvezi tipove iz podrazumevanog exporta (sigurno svuda)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Koristi sa browser SDK-om
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```