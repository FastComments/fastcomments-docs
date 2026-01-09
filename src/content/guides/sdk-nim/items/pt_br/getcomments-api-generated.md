---
## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| page | int | Não |  |
| limit | int | Não |  |
| skip | int | Não |  |
| asTree | bool | Não |  |
| skipChildren | int | Não |  |
| limitChildren | int | Não |  |
| maxTreeDepth | int | Não |  |
| urlId | string | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |
| contextUserId | string | Não |  |
| hashTag | string | Não |  |
| parentId | string | Não |  |
| direction | SortDirections | Não |  |

## Resposta

Retorna: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComments(
  tenantId = "my-tenant-123",
  page = 1,
  limit = 20,
  skip = 0,
  asTree = false,
  skipChildren = 0,
  limitChildren = 0,
  maxTreeDepth = 0,
  urlId = "news/2025-election-night",
  userId = "",
  anonUserId = "",
  contextUserId = "",
  hashTag = "",
  parentId = "",
  direction = SortDirections.Desc
)

if response.isSome:
  let comments = response.get()
  echo "Status: ", httpResponse.status, " Comments: ", comments
[inline-code-end]

---