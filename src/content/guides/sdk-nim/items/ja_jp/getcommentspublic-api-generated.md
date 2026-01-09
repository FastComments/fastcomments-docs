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

戻り値: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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