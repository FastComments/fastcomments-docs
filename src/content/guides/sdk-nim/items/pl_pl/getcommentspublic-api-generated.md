---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| page | int | Nie |  |
| direction | SortDirections | Nie |  |
| sso | string | Nie |  |
| skip | int | Nie |  |
| skipChildren | int | Nie |  |
| limit | int | Nie |  |
| limitChildren | int | Nie |  |
| countChildren | bool | Nie |  |
| fetchPageForCommentId | string | Nie |  |
| includeConfig | bool | Nie |  |
| countAll | bool | Nie |  |
| includei10n | bool | Nie |  |
| locale | string | Nie |  |
| modules | string | Nie |  |
| isCrawler | bool | Nie |  |
| includeNotificationCount | bool | Nie |  |
| asTree | bool | Nie |  |
| maxTreeDepth | int | Nie |  |
| useFullTranslationIds | bool | Nie |  |
| parentId | string | Nie |  |
| searchText | string | Nie |  |
| hashTags | seq[string] | Nie |  |
| userId | string | Nie |  |
| customConfigStr | string | Nie |  |
| afterCommentId | string | Nie |  |
| beforeCommentId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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