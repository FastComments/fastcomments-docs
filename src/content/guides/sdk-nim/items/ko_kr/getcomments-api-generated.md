---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | int | 아니오 |  |
| limit | int | 아니오 |  |
| skip | int | 아니오 |  |
| asTree | bool | 아니오 |  |
| skipChildren | int | 아니오 |  |
| limitChildren | int | 아니오 |  |
| maxTreeDepth | int | 아니오 |  |
| urlId | string | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |
| contextUserId | string | 아니오 |  |
| hashTag | string | 아니오 |  |
| parentId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| fromDate | int64 | 아니오 |  |
| toDate | int64 | 아니오 |  |

## 응답

반환: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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