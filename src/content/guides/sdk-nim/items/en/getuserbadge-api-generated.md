## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[APIGetUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_response.nim)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadge(tenantId = "my-tenant-123", id = "badge-9876")
if response.isSome:
  let badge = response.get()
  echo "Fetched badge:"
  echo badge
else:
  echo "No badge found"
  echo httpResponse
[inline-code-end]
