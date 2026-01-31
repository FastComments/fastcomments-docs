## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | int | No |  |
| tags | seq[string] | No |  |
| sso | string | No |  |
| isCrawler | bool | No |  |
| includeUserInfo | bool | No |  |

## Response

Returns: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Example

[inline-code-attrs-start title = 'getFeedPostsPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  echo "Retrieved feed posts:", feed
[inline-code-end]
