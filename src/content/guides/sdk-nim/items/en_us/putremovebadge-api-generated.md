## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgeId | string | No |  |
| userId | string | No |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Example

[inline-code-attrs-start title = 'putRemoveBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putRemoveBadge(badgeId = "verified-journalist",
  userId = "user-7890",
  commentId = "comment-98765",
  broadcastId = "",
  sso = "")

if response.isSome:
  let removeResp = response.get()
  discard removeResp
else:
  discard httpResponse
[inline-code-end]
