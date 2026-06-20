## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createFeedPostParams | CreateFeedPostParams | 否 |  |
| broadcastId | string | 否 |  |
| isLive | bool | 否 |  |
| doSpamCheck | bool | 否 |  |
| skipDupCheck | bool | 否 |  |

## 响应

返回：[`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## 示例

[inline-code-attrs-start title = 'createFeedPost 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let postParams = CreateFeedPostParams(
  title = "Major Acquisition by TechCorp",
  content = "TechCorp has acquired SoftWorks in a deal valued at $1.2B, creating a new market leader.",
  url = "news/tech/major-acquisition",
  tags = @["business", "technology"],
  authorId = "journalist-321"
)

let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = postParams,
  broadcastId = "",
  isLive = false,
  doSpamCheck = false,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---