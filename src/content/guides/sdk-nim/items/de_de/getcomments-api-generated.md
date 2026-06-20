## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| page | int | Nein |  |
| limit | int | Nein |  |
| skip | int | Nein |  |
| asTree | bool | Nein |  |
| skipChildren | int | Nein |  |
| limitChildren | int | Nein |  |
| maxTreeDepth | int | Nein |  |
| urlId | string | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |
| contextUserId | string | Nein |  |
| hashTag | string | Nein |  |
| parentId | string | Nein |  |
| direction | SortDirections | Nein |  |
| fromDate | int64 | Nein |  |
| toDate | int64 | Nein |  |

## Antwort

Gibt zurück: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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