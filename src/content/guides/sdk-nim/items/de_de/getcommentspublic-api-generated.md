## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int | Nein |  |
| direction | SortDirections | Nein |  |
| sso | string | Nein |  |
| skip | int | Nein |  |
| skipChildren | int | Nein |  |
| limit | int | Nein |  |
| limitChildren | int | Nein |  |
| countChildren | bool | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includeConfig | bool | Nein |  |
| countAll | bool | Nein |  |
| includei10n | bool | Nein |  |
| locale | string | Nein |  |
| modules | string | Nein |  |
| isCrawler | bool | Nein |  |
| includeNotificationCount | bool | Nein |  |
| asTree | bool | Nein |  |
| maxTreeDepth | int | Nein |  |
| useFullTranslationIds | bool | Nein |  |
| parentId | string | Nein |  |
| searchText | string | Nein |  |
| hashTags | seq[string] | Nein |  |
| userId | string | Nein |  |
| customConfigStr | string | Nein |  |
| afterCommentId | string | Nein |  |
| beforeCommentId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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