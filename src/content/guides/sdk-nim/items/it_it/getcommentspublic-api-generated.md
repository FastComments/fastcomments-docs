req
tenantId
urlId

## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
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

Restituisce: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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