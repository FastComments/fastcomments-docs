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
let reactParams = ReactBodyParams(reaction = "like", metadata = @["web", "homepage"], userId = "user-789")
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/2026/01/09/top-story",
  reactBodyParams = reactParams,
  isUndo = false,
  broadcastId = "broadcast-456",
  sso = "sso-token-xyz"
)
if response.isSome:
  let result = response.get()
  echo result
else:
  echo "No response body, HTTP status: ", httpResponse.status.code
[inline-code-end]
