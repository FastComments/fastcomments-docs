Lo SDK è scritto in TypeScript e fornisce definizioni di tipo complete per tutti i metodi API e i modelli di risposta:

```typescript
// Importa i tipi dall'export predefinito (sicuro ovunque)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Usa con lo SDK del browser
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```