## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| page | int | Não |  |
| direction | SortDirections | Não |  |
| sso | string | Não |  |
| skip | int | Não |  |
| skipChildren | int | Não |  |
| limit | int | Não |  |
| limitChildren | int | Não |  |
| countChildren | bool | Não |  |
| fetchPageForCommentId | string | Não |  |
| includeConfig | bool | Não |  |
| countAll | bool | Não |  |
| includei10n | bool | Não |  |
| locale | string | Não |  |
| modules | string | Não |  |
| isCrawler | bool | Não |  |
| includeNotificationCount | bool | Não |  |
| asTree | bool | Não |  |
| maxTreeDepth | int | Não |  |
| useFullTranslationIds | bool | Não |  |
| parentId | string | Não |  |
| searchText | string | Não |  |
| hashTags | seq[string] | Não |  |
| userId | string | Não |  |
| customConfigStr | string | Não |  |
| afterCommentId | string | Não |  |
| beforeCommentId | string | Não |  |

## Resposta

Retorna: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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