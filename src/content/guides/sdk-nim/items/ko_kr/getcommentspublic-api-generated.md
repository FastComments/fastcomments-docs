## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| page | int | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| sso | string | 아니오 |  |
| skip | int | 아니오 |  |
| skipChildren | int | 아니오 |  |
| limit | int | 아니오 |  |
| limitChildren | int | 아니오 |  |
| countChildren | bool | 아니오 |  |
| fetchPageForCommentId | string | 아니오 |  |
| includeConfig | bool | 아니오 |  |
| countAll | bool | 아니오 |  |
| includei10n | bool | 아니오 |  |
| locale | string | 아니오 |  |
| modules | string | 아니오 |  |
| isCrawler | bool | 아니오 |  |
| includeNotificationCount | bool | 아니오 |  |
| asTree | bool | 아니오 |  |
| maxTreeDepth | int | 아니오 |  |
| useFullTranslationIds | bool | 아니오 |  |
| parentId | string | 아니오 |  |
| searchText | string | 아니오 |  |
| hashTags | seq[string] | 아니오 |  |
| userId | string | 아니오 |  |
| customConfigStr | string | 아니오 |  |
| afterCommentId | string | 아니오 |  |
| beforeCommentId | string | 아니오 |  |

## 응답

반환: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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