## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| page | int | לא |  |
| limit | int | לא |  |
| skip | int | לא |  |
| asTree | bool | לא |  |
| skipChildren | int | לא |  |
| limitChildren | int | לא |  |
| maxTreeDepth | int | לא |  |
| urlId | string | כן |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |
| contextUserId | string | לא |  |
| hashTag | string | לא |  |
| parentId | string | לא |  |
| direction | SortDirections | לא |  |
| fromDate | int64 | לא |  |
| toDate | int64 | לא |  |

## תגובה

מחזיר: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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