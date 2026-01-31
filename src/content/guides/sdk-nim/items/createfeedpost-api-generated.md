## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| broadcastId | string | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| skipDupCheck | bool | No |  |

## Response

Returns: [`Option[CreateFeedPost_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post200response.nim)

## Example

[inline-code-attrs-start title = 'createFeedPost Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "Breaking: Nim SDK launches",
    content = "Announcing the new FastComments Nim SDK with feed post support.",
    authorId = "user-789",
    tags = @["nim", "fastcomments", "release"]
  ),
  broadcastId = "broadcast-456",
  isLive = false,
  doSpamCheck = true,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  echo "Created feed post:", $created
else:
  echo "Create feed post failed"
[inline-code-end]
