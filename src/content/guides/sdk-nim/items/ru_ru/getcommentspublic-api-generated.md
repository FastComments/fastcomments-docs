## Параметры

| Имя | Тип | Обязательно | Описание |
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

Возвращает: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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