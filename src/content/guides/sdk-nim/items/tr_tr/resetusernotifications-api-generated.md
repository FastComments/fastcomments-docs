## Parametreler

| Ad | Tip | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| options | ResetUserNotificationsOptions | Hayır |  |

## Yanıt

Returns: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Örnek

[inline-code-attrs-start title = 'resetUserNotifications Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  options = ResetUserNotificationsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]