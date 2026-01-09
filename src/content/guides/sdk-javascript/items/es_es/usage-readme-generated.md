Este SDK proporciona puntos de entrada separados para entornos de navegador y servidor para garantizar la compatibilidad y seguridad óptimas:

### Browser Usage (Client-Side)

Para aplicaciones de navegador/frontend, use la exportación segura para navegadores que excluye dependencias de Node.js:

```typescript
// Browser-safe import (no Node.js dependencies)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Create browser SDK instance
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Server Usage (Node.js)

Para aplicaciones de servidor/backend, use el SDK completo con SSO y características de autenticación:

```typescript
// Server-side import (includes SSO and designed to work with NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Create server SDK instance
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Keep this secret on the server!
  basePath: 'https://fastcomments.com' // optional, defaults to https://fastcomments.com
});

// Use secured APIs with your API key
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Types Only Import

Si solo necesita tipos de TypeScript (sin código en tiempo de ejecución), use la importación por defecto:

```typescript
// Types only (no runtime dependencies - safe everywhere)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Using Individual API Classes

#### Browser Environment

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Server Environment  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```