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
let (response, httpResponse) = client.updateFeedPost(
  tenantId = "my-tenant-123",
  id = "post-456",
  feedPost = FeedPost(
    id = "post-456",
    title = "Breaking News: Market Rally",
    content = "Stocks surged 5% after the Fed announcement; analysts cite renewed investor confidence.",
    published = true,
    tags = @["finance", "markets"]
  )
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]
