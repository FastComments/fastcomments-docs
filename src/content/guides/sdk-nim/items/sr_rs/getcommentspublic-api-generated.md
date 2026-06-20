---
req
tenantId
urlId

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | int | Не |  |
| direction | SortDirections | Не |  |
| sso | string | Не |  |
| skip | int | Не |  |
| skipChildren | int | Не |  |
| limit | int | Не |  |
| limitChildren | int | Не |  |
| countChildren | bool | Не |  |
| fetchPageForCommentId | string | Не |  |
| includeConfig | bool | Не |  |
| countAll | bool | Не |  |
| includei10n | bool | Не |  |
| locale | string | Не |  |
| modules | string | Не |  |
| isCrawler | bool | Не |  |
| includeNotificationCount | bool | Не |  |
| asTree | bool | Не |  |
| maxTreeDepth | int | Не |  |
| useFullTranslationIds | bool | Не |  |
| parentId | string | Не |  |
| searchText | string | Не |  |
| hashTags | seq[string] | Не |  |
| userId | string | Не |  |
| customConfigStr | string | Не |  |
| afterCommentId | string | Не |  |
| beforeCommentId | string | Не |  |

## Одговор

Враћа: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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