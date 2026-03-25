## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| usernameStartsWith | string | Não |  |
| mentionGroupIds | seq[string] | Não |  |
| sso | string | Não |  |
| searchSection | string | Não |  |

## Resposta

Retorna: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/2026/ai-product-launch",
  usernameStartsWith = "",
  mentionGroupIds = @[],
  sso = "",
  searchSection = ""
)
if response.isSome:
  let users = response.get()
  echo "Received users:", users.toString()
[inline-code-end]

---