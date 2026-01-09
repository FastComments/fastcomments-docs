---
## Παράμετροι

| Name | Type | Required | Description |
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

Επιστρέφει: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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