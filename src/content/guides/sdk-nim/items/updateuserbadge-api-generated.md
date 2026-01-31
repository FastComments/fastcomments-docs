## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateUserBadgeParams | UpdateUserBadgeParams | No |  |

## Response

Returns: [`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateUserBadgeParams = UpdateUserBadgeParams(
    badgeId = "badge-789",
    title = "Community Moderator",
    description = "Trusted moderator with elevated privileges",
    isActive = true,
    priority = 5,
    tags = @["moderator", "trusted"]
  )
)

if response.isSome:
  let updatedBadge = response.get()
  echo updatedBadge
[inline-code-end]
