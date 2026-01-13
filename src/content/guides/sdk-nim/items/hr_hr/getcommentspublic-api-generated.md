## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | int | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | int | Ne |  |
| skipChildren | int | Ne |  |
| limit | int | Ne |  |
| limitChildren | int | Ne |  |
| countChildren | bool | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | bool | Ne |  |
| countAll | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | bool | Ne |  |
| includeNotificationCount | bool | Ne |  |
| asTree | bool | Ne |  |
| maxTreeDepth | int | Ne |  |
| useFullTranslationIds | bool | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | seq[string] | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vraća: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'getCommentsPublic Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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