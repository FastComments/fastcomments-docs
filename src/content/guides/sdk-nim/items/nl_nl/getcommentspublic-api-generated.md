## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | int | Nee |  |
| skipChildren | int | Nee |  |
| limit | int | Nee |  |
| limitChildren | int | Nee |  |
| countChildren | bool | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | bool | Nee |  |
| countAll | bool | Nee |  |
| includei10n | bool | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | bool | Nee |  |
| includeNotificationCount | bool | Nee |  |
| asTree | bool | Nee |  |
| maxTreeDepth | int | Nee |  |
| useFullTranslationIds | bool | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | seq[string] | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Respons

Retourneert: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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