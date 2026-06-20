---
απαιτείται
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | int | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | int | Όχι |  |
| skipChildren | int | Όχι |  |
| limit | int | Όχι |  |
| limitChildren | int | Όχι |  |
| countChildren | bool | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | bool | Όχι |  |
| countAll | bool | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | bool | Όχι |  |
| includeNotificationCount | bool | Όχι |  |
| asTree | bool | Όχι |  |
| maxTreeDepth | int | Όχι |  |
| useFullTranslationIds | bool | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | seq[string] | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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