---
req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | int | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | int | Ні |  |
| skipChildren | int | Ні |  |
| limit | int | Ні |  |
| limitChildren | int | Ні |  |
| countChildren | bool | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | bool | Ні |  |
| countAll | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | bool | Ні |  |
| includeNotificationCount | bool | Ні |  |
| asTree | bool | Ні |  |
| maxTreeDepth | int | Ні |  |
| useFullTranslationIds | bool | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | seq[string] | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |

## Відповідь

Повертає: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  page = 2,
  direction = SortDirections.Descending,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 25,
  limitChildren = 5,
  countChildren = false,
  fetchPageForCommentId = "",
  includeConfig = true,
  countAll = false,
  includei10n = true,
  locale = "en-US",
  modules = "reactions,moderation",
  isCrawler = false,
  includeNotificationCount = true,
  asTree = true,
  maxTreeDepth = 3,
  useFullTranslationIds = false,
  parentId = "",
  searchText = "climate change",
  hashTags = @["climate", "research"],
  userId = "user-789",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let commentsResp = response.get()
  echo "Received comments response:"
  echo commentsResp
else:
  echo "No comments returned. HTTP status:", httpResponse.status
[inline-code-end]

---