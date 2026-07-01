---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| options | GetUserNotificationsOptions | Não |  |

## Resposta

Retorna: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getUserNotifications(tenantId = "my-tenant-123", options = GetUserNotificationsOptions())
if maybeResponse.isSome:
  let notifications = maybeResponse.get()
[inline-code-end]

---