---
req
tenantId
urlId

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

Retorna: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  page = 2,
  direction = SortDirections.Descending,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 25,
  limitChildren = 5,
  countChildren = false,
  fetchPageForCommentId = "",
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
  parentId = "",
  searchText = "climate change",
  hashTags = @["climate", "research"],
  userId = "user-789",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let commentsResp = response.get()
  echo "Received comments response:"
  echo commentsResp
else:
  echo "No comments returned. HTTP status:", httpResponse.status
[inline-code-end]

---