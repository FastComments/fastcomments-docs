---
Visualizadores atualmente online de uma página: pessoas cuja sessão WebSocket está inscrita na página neste momento.
Retorna anonCount + totalCount (assinantes da sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| afterName | string | Não |  |
| afterUserId | string | Não |  |

## Resposta

Retorna: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---