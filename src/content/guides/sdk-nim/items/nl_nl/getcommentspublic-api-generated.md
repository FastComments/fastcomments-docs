req
tenantId
urlId

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
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

Retourneert: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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