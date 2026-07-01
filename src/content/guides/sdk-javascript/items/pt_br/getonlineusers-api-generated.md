Atualmente, visualizadores online de uma página: pessoas cuja sessão websocket está inscrita na página neste exato momento.  
Retorna anonCount + totalCount (assinantes de toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| afterName | string | Não |  |
| afterUserId | string | Não |  |

## Resposta

Retorna: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Com parâmetros de paginação opcionais
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Sem parâmetros de paginação opcionais
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]