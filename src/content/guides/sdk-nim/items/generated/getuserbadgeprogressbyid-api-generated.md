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
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "badge-activity-456")
if response.isSome:
  let progress = response.get()
  echo "Retrieved badge progress for tenant my-tenant-123"
  discard progress
else:
  echo "No badge progress found, HTTP status: ", $httpResponse.status
[inline-code-end]
