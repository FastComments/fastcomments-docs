## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | int | Nej |  |
| skipChildren | int | Nej |  |
| limit | int | Nej |  |
| limitChildren | int | Nej |  |
| countChildren | bool | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | bool | Nej |  |
| countAll | bool | Nej |  |
| includei10n | bool | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | bool | Nej |  |
| includeNotificationCount | bool | Nej |  |
| asTree | bool | Nej |  |
| maxTreeDepth | int | Nej |  |
| useFullTranslationIds | bool | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | seq[string] | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |

## Svar

Returnerer: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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