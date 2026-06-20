req
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | int | Нет |  |
| direction | SortDirections | Нет |  |
| sso | string | Нет |  |
| skip | int | Нет |  |
| skipChildren | int | Нет |  |
| limit | int | Нет |  |
| limitChildren | int | Нет |  |
| countChildren | bool | Нет |  |
| fetchPageForCommentId | string | Нет |  |
| includeConfig | bool | Нет |  |
| countAll | bool | Нет |  |
| includei10n | bool | Нет |  |
| locale | string | Нет |  |
| modules | string | Нет |  |
| isCrawler | bool | Нет |  |
| includeNotificationCount | bool | Нет |  |
| asTree | bool | Нет |  |
| maxTreeDepth | int | Нет |  |
| useFullTranslationIds | bool | Нет |  |
| parentId | string | Нет |  |
| searchText | string | Нет |  |
| hashTags | seq[string] | Нет |  |
| userId | string | Нет |  |
| customConfigStr | string | Нет |  |
| afterCommentId | string | Нет |  |
| beforeCommentId | string | Нет |  |

## Ответ

Возвращает: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

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