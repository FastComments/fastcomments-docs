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
  title = "City Park Reopens",
  content = "Central Park reopens after a six-month renovation with new playgrounds and improved lighting.",
  tags = @["local","parks","community"],
  isPublished = true
)

let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/post-789",
  updateFeedPostParams = updateParams,
  broadcastId = "",
  sso = ""
)

if response.isSome:
  let post = response.get()
  echo "Updated post received: ", post
else:
  echo "Update failed, HTTP status: ", httpResponse.status
[inline-code-end]
