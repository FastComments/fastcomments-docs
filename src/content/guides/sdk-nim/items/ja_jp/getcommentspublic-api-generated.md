---
req
tenantId
urlId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| page | int | いいえ |  |
| direction | SortDirections | いいえ |  |
| sso | string | いいえ |  |
| skip | int | いいえ |  |
| skipChildren | int | いいえ |  |
| limit | int | いいえ |  |
| limitChildren | int | いいえ |  |
| countChildren | bool | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includeConfig | bool | いいえ |  |
| countAll | bool | いいえ |  |
| includei10n | bool | いいえ |  |
| locale | string | いいえ |  |
| modules | string | いいえ |  |
| isCrawler | bool | いいえ |  |
| includeNotificationCount | bool | いいえ |  |
| asTree | bool | いいえ |  |
| maxTreeDepth | int | いいえ |  |
| useFullTranslationIds | bool | いいえ |  |
| parentId | string | いいえ |  |
| searchText | string | いいえ |  |
| hashTags | seq[string] | いいえ |  |
| userId | string | いいえ |  |
| customConfigStr | string | いいえ |  |
| afterCommentId | string | いいえ |  |
| beforeCommentId | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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