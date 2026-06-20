## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| page | int | Ні |  |
| limit | int | Ні |  |
| skip | int | Ні |  |
| asTree | bool | Ні |  |
| skipChildren | int | Ні |  |
| limitChildren | int | Ні |  |
| maxTreeDepth | int | Ні |  |
| urlId | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |
| contextUserId | string | Ні |  |
| hashTag | string | Ні |  |
| parentId | string | Ні |  |
| direction | SortDirections | Ні |  |
| fromDate | int64 | Ні |  |
| toDate | int64 | Ні |  |

## Відповідь

Повертає: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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