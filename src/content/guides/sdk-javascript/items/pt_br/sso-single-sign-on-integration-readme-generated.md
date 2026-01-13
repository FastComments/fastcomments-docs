FastComments oferece suporte a SSO para integrar com seu sistema de autenticação de usuários existente. **A funcionalidade SSO está disponível apenas na exportação do servidor** já que requer recursos de crypto do Node.js.

### SSO Simples (Apenas no Lado do Servidor)

O SSO simples deve ser gerado no servidor e enviado ao cliente:

```typescript
// Código do lado do servidor (Node.js/back-end)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crie um SSO simples usando o helper embutido  
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

// Envie ssoToken para o código do cliente
// O código do lado do cliente pode então usar este token com o SDK para navegador
```

### SSO Seguro (Lado do Servidor, Recomendado)

O SSO seguro deve ser implementado no servidor e oferece maior segurança:

```typescript
// Código do lado do servidor (Node.js/back-end)
import { FastCommentsSSO, PublicApi } from 'fastcomments-sdk/server';

// Crie um SSO seguro usando o helper embutido
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

// Use com chamadas de API no servidor
const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: JSON.stringify(ssoConfig)
});

// Ou envie ssoConfig para o cliente para uso no navegador
```

### Usando SSO a partir do Navegador (com Token Gerado pelo Servidor)

```typescript
// Código do lado do cliente (navegador)
import { PublicApi } from 'fastcomments-sdk/browser';

// Obtenha o token SSO do endpoint do seu servidor
const ssoToken = await fetch('/api/sso-token').then(r => r.json());

const publicApi = new PublicApi();
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
  sso: ssoToken // Use o token SSO gerado no servidor
});
```

### SSO com Criação de Comentário

```typescript
// Lado do servidor: Crie SSO e comentário
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