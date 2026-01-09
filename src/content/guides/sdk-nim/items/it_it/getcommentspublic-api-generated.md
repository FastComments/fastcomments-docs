## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| page | int | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | int | No |  |
| skipChildren | int | No |  |
| limit | int | No |  |
| limitChildren | int | No |  |
| countChildren | bool | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | bool | No |  |
| countAll | bool | No |  |
| includei10n | bool | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | bool | No |  |
| includeNotificationCount | bool | No |  |
| asTree | bool | No |  |
| maxTreeDepth | int | No |  |
| useFullTranslationIds | bool | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | seq[string] | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Risposta

Restituisce: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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