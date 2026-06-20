req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
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

Returnerer: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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