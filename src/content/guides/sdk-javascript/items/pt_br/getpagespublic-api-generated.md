---
Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página.
Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| cursor | string | Não |  |
| limit | number | Não |  |
| q | string | Não |  |
| sortBy | PagesSortBy | Não |  |
| hasComments | boolean | Não |  |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---