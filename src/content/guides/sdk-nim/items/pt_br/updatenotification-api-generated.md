## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateNotificationBody | UpdateNotificationBody | Não |  |
| userId | string | Não |  |

## Resposta

Retorna: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-456",
  updateNotificationBody = UpdateNotificationBody(
    enabled = true,
    channels = @["email", "push"],
    frequency = "immediate"
  ),
  userId = "user-789"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---