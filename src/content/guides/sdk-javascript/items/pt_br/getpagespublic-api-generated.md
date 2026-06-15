Lista as páginas de um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página.
Páginas que exigem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| cursor | string | Não |  |
| limit | number | Não |  |
| q | string | Não |  |
| sortBy | PagesSortBy | Não |  |
| hasComments | boolean | Não |  |

## Response

Retorna: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---