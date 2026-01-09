---
Das SDK ist in TypeScript geschrieben und stellt vollständige Typdefinitionen für alle API-Methoden und Antwortmodelle bereit:

```typescript
// Typen aus dem Standardexport importieren (überall sicher)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Mit dem Browser-SDK verwenden
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```
---