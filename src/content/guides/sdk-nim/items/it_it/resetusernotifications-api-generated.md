## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | ResetUserNotificationsOptions | No |  |

## Risposta

Restituisce: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Esempio

[inline-code-attrs-start title = 'resetUserNotifications Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  options = ResetUserNotificationsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]