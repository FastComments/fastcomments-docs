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
let updateParams = UpdateUserBadgeParams(
  name = "Top Contributor",
  color = "#FFD700",
  icon = "star",
  isActive = true,
  rank = 1,
  tags = @["community", "trusted"]
)

let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "badge-987",
  updateUserBadgeParams = updateParams
)

if response.isSome:
  let updated = response.get()
  echo "Updated badge: ", updated
[inline-code-end]
