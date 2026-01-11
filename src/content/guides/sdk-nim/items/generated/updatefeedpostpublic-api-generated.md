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
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-9876",
  updateFeedPostParams = UpdateFeedPostParams(
    title = "Breaking: Major Update on Community Project",
    content = "Phase 2 completed with improvements to accessibility and performance.",
    isPinned = false,
    tags = @["community", "update"],
    authorId = "editor-42"
  ),
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let postResp = response.get()
  discard postResp
[inline-code-end]
