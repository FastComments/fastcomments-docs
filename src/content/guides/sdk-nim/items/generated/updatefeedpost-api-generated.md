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
  id = "feedpost-456",
  feedPost = FeedPost(
    title = "Downtown Festival 2025",
    body = "Join us for the annual downtown festival with live music, food trucks, and family activities.",
    url = "news/downtown-festival-2025",
    author = "editor@localnews.com",
    tags = @["community", "festival"],
    published = true
  )
)

if response.isSome:
  let result = response.get()
  echo "FlagCommentPublic response received: ", result
else:
  echo "No response returned, HTTP status: ", httpResponse.status
[inline-code-end]
