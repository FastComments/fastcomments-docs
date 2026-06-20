req
tenantId
urlId

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

Zwraca: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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