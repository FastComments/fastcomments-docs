## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| reactBodyParams | ReactBodyParams | No |  |
| isUndo | bool | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## Example

[inline-code-attrs-start title = 'reactFeedPostPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let reactBody = ReactBodyParams(reaction = "like", metadata = @["section:news"])
let (response, httpResponse) = client.reactFeedPostPublic(tenantId = "my-tenant-123", postId = "news/article-title-456", reactBodyParams = reactBody, isUndo = false, broadcastId = "broadcast-789", sso = "user-42-sso-token")
if response.isSome:
  let result = response.get()
  echo "Reaction recorded:", result
else:
  echo "No response body, HTTP status:", httpResponse.status
[inline-code-end]
