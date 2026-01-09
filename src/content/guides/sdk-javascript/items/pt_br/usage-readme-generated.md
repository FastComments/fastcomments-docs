Este SDK fornece pontos de entrada separados para ambientes de navegador e servidor para garantir compatibilidade e segurança ideais:

### Uso no Navegador (Lado do Cliente)

Para aplicações de frontend/navegador, use a exportação segura para navegador que exclui dependências do Node.js:

```typescript
// Import seguro para navegador (sem dependências do Node.js)
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// Criar instância do SDK para navegador
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // opcional, padrão https://fastcomments.com
});

// Usar APIs públicas (sem necessidade de chave de API - seguro para navegadores)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Uso no Servidor (Node.js)

Para aplicações de servidor/backend, use o SDK completo com recursos de SSO e autenticação:

```typescript
// Import para servidor (inclui SSO e foi projetado para funcionar com NodeJS)
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// Criar instância do SDK para servidor
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // Mantenha isto em segredo no servidor!
  basePath: 'https://fastcomments.com' // opcional, padrão https://fastcomments.com
});

// Use APIs seguras com sua chave de API
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### Import apenas de Tipos

Se você precisa apenas dos tipos TypeScript (sem código em tempo de execução), use o import padrão:

```typescript
// Somente tipos (sem dependências em tempo de execução - seguro em qualquer lugar)
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### Usando Classes de API Individuais

#### Ambiente do Navegador

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### Ambiente do Servidor  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```