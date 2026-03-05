## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| badgeId | string | No |  |
| displayedOnComments | bool | No |  |
| limit | float64 | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadges Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadges(
  tenantId = "my-tenant-123",
  userId = "user-456",
  badgeId = "contributor",
  displayedOnComments = true,
  limit = 25.0,
  skip = 0.0
)

if response.isSome:
  let badges = response.get()
  echo "User badges retrieved"
else:
  echo "No badges found; HTTP status: ", httpResponse.status
[inline-code-end]
