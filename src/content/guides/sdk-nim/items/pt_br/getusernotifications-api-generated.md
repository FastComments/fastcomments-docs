---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| pageSize | int | Não |  |
| afterId | string | Não |  |
| includeContext | bool | Não |  |
| afterCreatedAt | int64 | Não |  |
| unreadOnly | bool | Não |  |
| dmOnly | bool | Não |  |
| noDm | bool | Não |  |
| includeTranslations | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 50,
  afterId = "notif_9a1b2c3d",
  includeContext = true,
  afterCreatedAt = int64(1699999999000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  sso = ""
)
if response.isSome:
  let notifications = response.get()
  discard notifications
else:
  discard httpResponse
[inline-code-end]

---