## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | SearchUsersOptions | Não |  |

## Resposta

Retorna: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = SearchUsersOptions()
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---