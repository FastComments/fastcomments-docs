## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| afterCreatedAt | int64 | Não |  |
| unreadOnly | bool | Não |  |
| dmOnly | bool | Não |  |
| noDm | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = int64(0),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---