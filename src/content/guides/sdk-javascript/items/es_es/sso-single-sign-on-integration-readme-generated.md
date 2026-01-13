FastComments admite SSO para integrarse con su sistema de autenticación de usuarios existente. **La funcionalidad SSO solo está disponible en la exportación del servidor** ya que requiere funciones criptográficas de Node.js.

### SSO simple (solo en el servidor)

El SSO simple debe generarse en el servidor y enviarse al cliente:

```typescript
// Código del lado del servidor (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crear SSO simple usando el auxiliar integrado  
const userData = {
  username: 'john_doe',
  email: 'john@example.com',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg'
};

const sso = FastCommentsSSO.createSimple(userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoToken = sso.createToken();

// Envíe ssoToken a su código del lado del cliente
// El código del lado del cliente puede entonces usar este token con el SDK para navegador
```

### SSO seguro (lado del servidor, recomendado)

El SSO seguro debe implementarse en el servidor y ofrece mayor seguridad:

```typescript
// Código del lado del servidor (Node.js/backend)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crear SSO seguro usando el auxiliar integrado
const userData = {
  id: 'user-123',
  email: 'john@example.com',
  username: 'john_doe',
  displayName: 'John Doe',
  avatar: 'https://example.com/avatar.jpg',
  isAdmin: false,
  isModerator: false
};

const sso = FastCommentsSSO.createSecure('your-api-key', userData, {
  loginURL: '/login',
  logoutURL: '/logout'
});

const ssoConfig = sso.prepareToSend();

// Usar con llamadas a la API en el servidor
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// O enviar ssoConfig al cliente para uso en el navegador
```

### Uso de SSO desde el navegador (con token generado por el servidor)

```typescript
// Código del lado del cliente (navegador)
import { PublicApi } from 'fastcomments-sdk/browser';

// Obtener el token SSO desde su endpoint en el servidor
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Usar el token SSO generado por el servidor
});
```

### SSO con creación de comentarios

```typescript
// Lado del servidor: crear SSO y comentario
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

const sso = FastCommentsSSO.createSecure('your-api-key', userData);
const ssoConfig = sso.prepareToSend();

const response = await publicApi.createCommentPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  broadcastId: 'unique-broadcast-id',
  commentData: {
    comment: 'This is my comment',
    date: Date.now(),
    commenterName: 'John Doe',
    url: 'https://example.com/page',
    urlId: 'page-url-id'
  },
  sso: JSON.stringify(ssoConfig)
});
```