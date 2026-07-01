## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## Response

반환값: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Example

[inline-code-attrs-start title = 'getUserNotificationCount 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]