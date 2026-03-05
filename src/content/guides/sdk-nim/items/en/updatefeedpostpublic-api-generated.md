## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| updateFeedPostParams | UpdateFeedPostParams | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateFeedPostParams(
  title = "Launch: FastComments v2.0",
  content = "Announcing the v2.0 release with performance and moderation improvements.",
  visible = true,
  tags = @["release", "announcement"],
  priority = 1
)

let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/launch-fastcomments-v2",
  updateFeedPostParams = updateParams,
  broadcastId = "",
  sso = ""
)

if response.isSome:
  let updated = response.get()
  echo "Feed post updated successfully"
[inline-code-end]
