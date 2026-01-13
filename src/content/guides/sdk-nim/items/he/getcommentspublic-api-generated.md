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

מחזיר: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/world/article-2025",
  page = 1,
  direction = SortDirections(0),
  sso = "sso_token_abc",
  skip = 0,
  skipChildren = 0,
  limit = 20,
  limitChildren = 5,
  countChildren = false,
  fetchPageForCommentId = "cmt_789",
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
  parentId = "parent_123",
  searchText = "openAI integration",
  hashTags = @["ai", "technology"],
  userId = "user_456",
  customConfigStr = "{}",
  afterCommentId = "cmt_100",
  beforeCommentId = ""
)

if response.isSome:
  let comments = response.get()
  discard comments
else:
  discard httpResponse
[inline-code-end]

---