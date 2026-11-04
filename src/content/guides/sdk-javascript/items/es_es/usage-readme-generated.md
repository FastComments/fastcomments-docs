Este SDK proporciona puntos de entrada separados para entornos de navegador y servidor para garantizar una compatibilidad y seguridad óptimas:

### Uso en el navegador (lado del cliente)

Para aplicaciones de navegador/frontend, utilice la exportación segura para el navegador que excluye las dependencias de Node.js:

```typescript
// Importación segura para el navegador (sin dependencias de Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Crear instancia del SDK para navegador
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opcional, por defecto es https://fastcomments.com
});

// Usar APIs públicas (no se necesita clave de API - seguro para navegadores)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Uso en el servidor (Node.js)

Para aplicaciones de servidor/backend, utilice el SDK completo con funciones de SSO y autenticación:

```typescript
// Importación del lado del servidor (incluye SSO y está diseñado para funcionar con NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Crear instancia del SDK para servidor
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // ¡Mantenga esto en secreto en el servidor!
  basePath: 'https://fastcomments.com' // opcional, por defecto es https://fastcomments.com
});

// Usar APIs protegidas con su clave de API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Importación solo de tipos

Si solo necesita los tipos de TypeScript (sin código en tiempo de ejecución), utilice la importación por defecto:

```typescript
// Solo tipos (sin dependencias en tiempo de ejecución - seguro en todas partes)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Uso de clases de API individuales

#### Entorno de navegador

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Entorno de servidor  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```