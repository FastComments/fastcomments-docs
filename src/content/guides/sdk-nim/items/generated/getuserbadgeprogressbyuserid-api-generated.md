## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`Option[GetUserBadgeProgressById_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_by_id200response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if response.isSome:
  let badgeProgress = response.get()
  echo "Received badge progress for user: ", "user-456"
else:
  echo "No badge progress found, HTTP status: ", $httpResponse.status
[inline-code-end]
