## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de unFlagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(
  tenantId = "my-tenant-123",
  id = "flag-789",
  userId = "",
  anonUserId = ""
)

if response.isSome:
  let flagResponse = response.get()
  echo "Comment unflagged successfully"
[inline-code-end]

---