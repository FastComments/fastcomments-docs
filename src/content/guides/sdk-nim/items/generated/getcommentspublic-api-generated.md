## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| page | int | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | int | No |  |
| skipChildren | int | No |  |
| limit | int | No |  |
| limitChildren | int | No |  |
| countChildren | bool | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | bool | No |  |
| countAll | bool | No |  |
| includei10n | bool | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | bool | No |  |
| includeNotificationCount | bool | No |  |
| asTree | bool | No |  |
| maxTreeDepth | int | No |  |
| useFullTranslationIds | bool | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | seq[string] | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Response

Returns: [`Option[GetCommentsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_public200response.nim)

## Example

[inline-code-attrs-start title = 'getCommentsPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/2025/important-update",
  page = 1,
  direction = SortDirections.desc,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 20,
  limitChildren = 5,
  countChildren = false,
  fetchPageForCommentId = "",
  includeConfig = true,
  countAll = false,
  includei10n = false,
  locale = "en-US",
  modules = "",
  isCrawler = false,
  includeNotificationCount = true,
  asTree = true,
  maxTreeDepth = 3,
  useFullTranslationIds = false,
  parentId = "",
  searchText = "",
  hashTags = @["breaking", "announcement"],
  userId = "",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let comments = response.get()
  echo "Fetched comments: ", comments
[inline-code-end]
