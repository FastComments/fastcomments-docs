## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | int | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | int | Non |  |
| skipChildren | int | Non |  |
| limit | int | Non |  |
| limitChildren | int | Non |  |
| countChildren | bool | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | bool | Non |  |
| countAll | bool | Non |  |
| includei10n | bool | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | bool | Non |  |
| includeNotificationCount | bool | Non |  |
| asTree | bool | Non |  |
| maxTreeDepth | int | Non |  |
| useFullTranslationIds | bool | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | seq[string] | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Retourne: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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