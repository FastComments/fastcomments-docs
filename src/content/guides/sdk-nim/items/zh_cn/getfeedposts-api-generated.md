## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | int | 否 |  |
| tags | seq[string] | 否 |  |

## 响应

返回: [`Option[GetFeedPosts_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts200response.nim)

## 示例

[inline-code-attrs-start title = 'getFeedPosts 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPosts(tenantId = "my-tenant-123", afterId = "post_abc123", limit = 20, tags = @["news", "sports"])
if response.isSome:
  let feed = response.get()
  echo "Feed posts retrieved for tenant my-tenant-123"
else:
  echo "No feed posts returned, HTTP status: ", $httpResponse.status
[inline-code-end]