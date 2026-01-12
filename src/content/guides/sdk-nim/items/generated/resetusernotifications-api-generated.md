## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| afterCreatedAt | int64 | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = 0'i64,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)
if response.isSome:
  let resetResult = response.get()
  echo "ResetUserNotifications succeeded: ", resetResult
else:
  echo "ResetUserNotifications returned no body, status: ", httpResponse.status
[inline-code-end]
