Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Members".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | GetOfflineUsersOptions | Não |  |

## Resposta

Retorna: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]