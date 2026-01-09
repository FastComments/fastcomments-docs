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

## תגובה

מחזיר: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---