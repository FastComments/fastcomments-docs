## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | int | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | int | 否 |  |
| skipChildren | int | 否 |  |
| limit | int | 否 |  |
| limitChildren | int | 否 |  |
| countChildren | bool | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | bool | 否 |  |
| countAll | bool | 否 |  |
| includei10n | bool | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | bool | 否 |  |
| includeNotificationCount | bool | 否 |  |
| asTree | bool | 否 |  |
| maxTreeDepth | int | 否 |  |
| useFullTranslationIds | bool | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | seq[string] | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |

## 响应

返回：[`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## 示例

[inline-code-attrs-start title = 'getCommentsPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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