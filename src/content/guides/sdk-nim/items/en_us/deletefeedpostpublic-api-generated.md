## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[DeleteFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public200response.nim)

## Example

[inline-code-attrs-start title = 'deleteFeedPostPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  broadcastId = "broadcast-789",
  sso = ""
)
if response.isSome:
  let result = response.get()
[inline-code-end]