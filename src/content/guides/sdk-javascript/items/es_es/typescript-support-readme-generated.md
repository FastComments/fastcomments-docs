El SDK está escrito en TypeScript y proporciona definiciones de tipos completas para todos los métodos de la API y los modelos de respuesta:

```typescript
// Importa tipos desde la exportación por defecto (seguro en cualquier lugar)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';

// Usar con el SDK del navegador
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

const sdk = createFastCommentsBrowserSDK();
const response: GetCommentsPublic200Response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

const comments: PublicComment[] = response.comments || [];
```