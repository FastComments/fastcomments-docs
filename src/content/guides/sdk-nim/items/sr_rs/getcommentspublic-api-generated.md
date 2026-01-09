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

Враћа: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Пример

[inline-code-attrs-start title = 'getCommentsPublic Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/world/article-2025",
  page = 1,
  direction = SortDirections(0),
  sso = "sso_token_abc",
  skip = 0,
  skipChildren = 0,
  limit = 20,
  limitChildren = 5,
  countChildren = false,
  fetchPageForCommentId = "cmt_789",
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
  parentId = "parent_123",
  searchText = "openAI integration",
  hashTags = @["ai", "technology"],
  userId = "user_456",
  customConfigStr = "{}",
  afterCommentId = "cmt_100",
  beforeCommentId = ""
)

if response.isSome:
  let comments = response.get()
  discard comments
else:
  discard httpResponse
[inline-code-end]