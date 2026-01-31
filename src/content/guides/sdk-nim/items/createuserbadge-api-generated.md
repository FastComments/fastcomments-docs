## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | No |  |

## Response

Returns: [`Option[CreateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_user_badge200response.nim)

## Example

[inline-code-attrs-start title = 'createUserBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = CreateUserBadgeParams(
  badgeId = "contributor-100",
  name = "Top Contributor",
  description = "Awarded for 100 meaningful comments",
  iconUrl = "https://cdn.myapp.com/badges/top-contributor.png",
  isActive = true,
  tags = @["community", "engagement"],
  points = 100
)

let (response, httpResponse) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = createParams)

if response.isSome:
  let badge = response.get()
  echo "Created badge:", badge.id, " name:", badge.name, " points:", $badge.points
[inline-code-end]
