---
## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | int | Nej |  |
| limit | int | Nej |  |
| skip | int | Nej |  |
| asTree | bool | Nej |  |
| skipChildren | int | Nej |  |
| limitChildren | int | Nej |  |
| maxTreeDepth | int | Nej |  |
| urlId | string | Ja |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |
| contextUserId | string | Nej |  |
| hashTag | string | Nej |  |
| parentId | string | Nej |  |
| direction | SortDirections | Nej |  |
| fromDate | int64 | Nej |  |
| toDate | int64 | Nej |  |

## Respons

Returnerer: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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