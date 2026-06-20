## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| page | int | Nee |  |
| limit | int | Nee |  |
| skip | int | Nee |  |
| asTree | bool | Nee |  |
| skipChildren | int | Nee |  |
| limitChildren | int | Nee |  |
| maxTreeDepth | int | Nee |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |
| contextUserId | string | Nee |  |
| hashTag | string | Nee |  |
| parentId | string | Nee |  |
| direction | SortDirections | Nee |  |
| fromDate | int64 | Nee |  |
| toDate | int64 | Nee |  |

## Antwoord

Geeft terug: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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