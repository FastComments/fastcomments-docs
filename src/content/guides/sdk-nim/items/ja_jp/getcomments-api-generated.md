---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | int | いいえ |  |
| limit | int | いいえ |  |
| skip | int | いいえ |  |
| asTree | bool | いいえ |  |
| skipChildren | int | いいえ |  |
| limitChildren | int | いいえ |  |
| maxTreeDepth | int | いいえ |  |
| urlId | string | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |
| contextUserId | string | いいえ |  |
| hashTag | string | いいえ |  |
| parentId | string | いいえ |  |
| direction | SortDirections | いいえ |  |
| fromDate | int64 | いいえ |  |
| toDate | int64 | いいえ |  |

## レスポンス

戻り値: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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