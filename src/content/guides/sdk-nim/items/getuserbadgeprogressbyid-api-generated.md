## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetUserBadgeProgressById_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_by_id200response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let badgeId = "badge-456"
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = tenantId, id = badgeId)
if response.isSome:
  let badgeProgress = response.get()
  echo "Got badge progress for ", tenantId
  echo badgeProgress
else:
  echo "Badge progress not found for ", tenantId
[inline-code-end]
