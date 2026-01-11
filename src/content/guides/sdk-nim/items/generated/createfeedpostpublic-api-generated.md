## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "New Feature: Real-time Comments",
    content = "FastComments now supports real-time threading and moderation across articles.",
    authorId = "editor-42",
    tags = @["product","release","comments"],
    isPublished = true
  ),
  broadcastId = "",
  sso = ""
)

if response.isSome:
  let created = response.get()
  echo "Created feed post:", created
[inline-code-end]
