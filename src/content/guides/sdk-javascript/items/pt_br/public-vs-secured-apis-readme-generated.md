O SDK fornece três classes principais de API:

- **`DefaultApi`** - Endpoints protegidos que exigem sua chave de API para autenticação. Use-os para operações do lado do servidor.
- **`PublicApi`** - Endpoints públicos que podem ser acessados sem uma chave de API. Podem ser chamados diretamente de navegadores/dispositivos móveis/etc.
- **`HiddenApi`** - Endpoints internos/administrador para casos de uso avançados.

### Exemplo: Usando a `PublicApi` (seguro para navegador)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Obter comentários de uma página (sem necessidade de chave de API)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Exemplo: Usando a `DefaultApi` (somente no servidor)

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