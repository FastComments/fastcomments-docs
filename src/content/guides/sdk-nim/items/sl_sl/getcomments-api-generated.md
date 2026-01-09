## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | int | Ne |  |
| limit | int | Ne |  |
| skip | int | Ne |  |
| asTree | bool | Ne |  |
| skipChildren | int | Ne |  |
| limitChildren | int | Ne |  |
| maxTreeDepth | int | Ne |  |
| urlId | string | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |
| contextUserId | string | Ne |  |
| hashTag | string | Ne |  |
| parentId | string | Ne |  |
| direction | SortDirections | Ne |  |

## Odgovor

Vrne: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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