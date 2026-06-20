req
tenantId
urlId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | int | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | int | לא |  |
| skipChildren | int | לא |  |
| limit | int | לא |  |
| limitChildren | int | לא |  |
| countChildren | bool | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | bool | לא |  |
| countAll | bool | לא |  |
| includei10n | bool | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | bool | לא |  |
| includeNotificationCount | bool | לא |  |
| asTree | bool | לא |  |
| maxTreeDepth | int | לא |  |
| useFullTranslationIds | bool | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | seq[string] | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה

מחזיר: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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