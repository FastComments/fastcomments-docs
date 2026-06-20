## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-sso-token-456")
if response.isSome:
  let result = response.get()
  echo "ResetUserNotificationsResponse:", result
else:
  echo "Reset failed, HTTP response:", httpResponse
[inline-code-end]

---