## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | int | Non |  |
| limit | int | Non |  |
| skip | int | Non |  |
| asTree | bool | Non |  |
| skipChildren | int | Non |  |
| limitChildren | int | Non |  |
| maxTreeDepth | int | Non |  |
| urlId | string | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |
| contextUserId | string | Non |  |
| hashTag | string | Non |  |
| parentId | string | Non |  |
| direction | SortDirections | Non |  |
| fromDate | int64 | Non |  |
| toDate | int64 | Non |  |

## Réponse

Retourne : [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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