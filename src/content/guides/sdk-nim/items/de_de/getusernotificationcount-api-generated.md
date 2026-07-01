## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## Antwort

Rückgabe: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getUserNotificationCount Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]