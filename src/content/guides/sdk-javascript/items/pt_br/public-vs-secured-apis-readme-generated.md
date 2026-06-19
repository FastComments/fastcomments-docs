O SDK fornece as seguintes classes de API:

- **`DefaultApi`** - Endpoints seguros que exigem sua chave de API para autenticação. Use-os para operações do lado do servidor.
- **`PublicApi`** - Endpoints públicos que podem ser acessados sem uma chave de API. Podem ser chamados diretamente de navegadores/dispositivos móveis/etc.
- **`ModerationApi`** - Endpoints do painel de moderação (moderação de comentários, banimentos, badges, fator de confiança, busca). Autenticados pela sessão do moderador; passe o parâmetro de consulta `sso` para moderadores autenticados via SSO.
- **`HiddenApi`** - Endpoints internos/administrativos para casos de uso avançados.

### Exemplo: Usando a Public API (compatível com navegador)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Obter comentários de uma página (nenhuma chave de API necessária)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemplo: Usando a Default API (somente no servidor)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Mantenha esta chave em segredo!
});
const defaultApi = new DefaultApi(config);

// Obter comentários com acesso administrativo completo
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemplo: Usando a Moderation API

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Chamadas autenticadas por moderador (cookie de sessão, ou passe `sso` para um moderador autenticado via SSO).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```