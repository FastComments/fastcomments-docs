El SDK proporciona estas clases de API:

- **`DefaultApi`** - Endpoints seguros que requieren tu clave de API para la autenticación. Úsalos para operaciones del lado del servidor.
- **`PublicApi`** - Endpoints públicos a los que se puede acceder sin una clave de API. Pueden llamarse directamente desde navegadores/dispositivos móviles/etc.
- **`ModerationApi`** - Endpoints del panel de moderación (moderación de comentarios, expulsiones, insignias, factor de confianza, búsqueda). Autenticados mediante la sesión del moderador; pasa el parámetro de consulta `sso` para moderadores autenticados por SSO.
- **`HiddenApi`** - Endpoints internos/administrativos para casos de uso avanzados.

### Ejemplo: Uso de la API pública (seguro para navegador)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Obtener comentarios para una página (no se requiere clave de API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Ejemplo: Uso de la API predeterminada (solo del lado del servidor)

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

### Ejemplo: Uso de la API de moderación

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Llamadas autenticadas por moderador (cookie de sesión, o pasar `sso` para un moderador SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```