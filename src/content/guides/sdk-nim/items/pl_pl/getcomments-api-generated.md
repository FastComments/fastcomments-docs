---
## Parametry

| Name | Type | Required | Description |
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
| fromDate | int64 | Nie |  |
| toDate | int64 | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComments(
  tenantId = "my-tenant-123",
  page = 1,
  limit = 25,
  skip = 0,
  asTree = true,
  skipChildren = 0,
  limitChildren = 5,
  maxTreeDepth = 3,
  urlId = "news/2026-global-economy",
  userId = "user-789",
  anonUserId = "",
  contextUserId = "",
  hashTag = "economy",
  parentId = "",
  direction = SortDirections.Desc,
  fromDate = 1710000000000'i64,
  toDate = 1710100000000'i64
)
if response.isSome:
  let commentsResp = response.get()
  discard commentsResp
[inline-code-end]
---