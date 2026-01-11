## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetUserNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count200response.nim)

## Example

[inline-code-attrs-start title = 'getUserNotificationCount Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummySignature")
if response.isSome:
  let countResp = response.get()
  discard countResp
else:
  discard httpResponse
[inline-code-end]
