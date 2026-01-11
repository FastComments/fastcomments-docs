## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlIdWS | string | No |  |
| userIds | string | No |  |

## Response

Returns: [`Option[GetUserPresenceStatuses_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_presence_statuses200response.nim)

## Example

[inline-code-attrs-start title = 'getUserPresenceStatuses Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserPresenceStatuses(tenantId = "my-tenant-123", urlIdWS = "", userIds = "")

if response.isSome:
  let statuses = response.get()
  echo "Presence statuses: ", $statuses
else:
  echo "No presence statuses returned"
[inline-code-end]
