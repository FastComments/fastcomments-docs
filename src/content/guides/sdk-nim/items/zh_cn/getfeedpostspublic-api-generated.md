---
req
tenantId
afterId

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | int | 否 |  |
| tags | seq[string] | 否 |  |
| sso | string | 否 |  |
| isCrawler | bool | 否 |  |
| includeUserInfo | bool | 否 |  |

## 响应

返回: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## 示例

[inline-code-attrs-start title = 'getFeedPostsPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)
if response.isSome:
  let feed = response.get()
  discard feed
[inline-code-end]

---