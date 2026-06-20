## Parâmetros

| Nome | Type | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| afterCreatedAt | int64 | Não |  |
| unreadOnly | bool | Não |  |
| dmOnly | bool | Não |  |
| noDm | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  let resetResp = response.get()
  echo "ResetUserNotificationsResponse received"
else:
  echo "No ResetUserNotificationsResponse"
[inline-code-end]

---