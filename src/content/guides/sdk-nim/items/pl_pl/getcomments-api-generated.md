## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| page | int | Nie |  |
| limit | int | Nie |  |
| skip | int | Nie |  |
| asTree | bool | Nie |  |
| skipChildren | int | Nie |  |
| limitChildren | int | Nie |  |
| maxTreeDepth | int | Nie |  |
| urlId | string | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |
| contextUserId | string | Nie |  |
| hashTag | string | Nie |  |
| parentId | string | Nie |  |
| direction | SortDirections | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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