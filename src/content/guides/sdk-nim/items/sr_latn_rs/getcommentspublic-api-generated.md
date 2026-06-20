---
zahtev
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | int | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | int | Ne |  |
| skipChildren | int | Ne |  |
| limit | int | Ne |  |
| limitChildren | int | Ne |  |
| countChildren | bool | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | bool | Ne |  |
| countAll | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | bool | Ne |  |
| includeNotificationCount | bool | Ne |  |
| asTree | bool | Ne |  |
| maxTreeDepth | int | Ne |  |
| useFullTranslationIds | bool | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | seq[string] | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vraća: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Primer

[inline-code-attrs-start title = 'getCommentsPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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