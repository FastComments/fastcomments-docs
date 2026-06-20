req
tenantId
urlId

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

Gibt zurück: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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