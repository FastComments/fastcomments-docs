req
tenantId
urlId

## Paramètres

| Name | Type | Required | Description |
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

Renvoie: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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