## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | float64 | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetUserBadgeProgressList_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_list200response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressList Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressList(tenantId = "my-tenant-123", userId = "user-9823", limit = 25.0, skip = 0.0)
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress received:", badgeProgress
else:
  echo "No badge progress. HTTP response:", httpResponse.status
[inline-code-end]