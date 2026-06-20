## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| notificationId | string | Não |  |
| newStatus | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  newStatus = "read",
  sso = "sso-token-abc123"
)
if response.isSome:
  let updated = response.get()
  echo "Notification status updated successfully"
else:
  echo "No update response received"
[inline-code-end]

---