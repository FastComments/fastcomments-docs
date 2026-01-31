## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| feedPost | FeedPost | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateFeedPost Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPost(tenantId = "my-tenant-123",
  id = "feedpost-456",
  feedPost = FeedPost(title = "Breaking News: Market Rally",
    content = "Markets rallied today as investors cheered strong earnings.",
    tags = @["finance", "markets"],
    pinned = false,
    authorId = "editor-99",
    published = true,
    views = 0))

if response.isSome:
  let flagResp = response.get()
  echo "Update successful:", flagResp
else:
  echo "No response body, HTTP status: ", httpResponse.status
[inline-code-end]
