El SDK proporciona tres clases principales de API:

- **`DefaultApi`** - Endpoints seguros que requieren su clave de API para la autenticación. Úselos para operaciones del lado del servidor.
- **`PublicApi`** - Endpoints públicos a los que se puede acceder sin una clave de API. Pueden llamarse directamente desde navegadores, dispositivos móviles, etc.
- **`HiddenApi`** - Endpoints internos/administrativos para casos de uso avanzados.

### Ejemplo: Uso de la API pública (segura para navegadores)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Obtener comentarios de una página (no se requiere clave de API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Ejemplo: Uso de Default API (solo del lado del servidor)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // ¡Mantén esto en secreto!
});
const defaultApi = new DefaultApi(config);

// Obtener comentarios con acceso administrativo completo
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```