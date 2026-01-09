## Параметри

| Name | Type | Required | Description |
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

Повертає: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---