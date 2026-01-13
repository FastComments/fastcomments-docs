## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| urlId | string | Sim |  |
| fromCommentId | string | Não |  |
| viewed | bool | Não |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[GetNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(
  tenantId = "fastcomments-tenant-42",
  userId = "",
  urlId = "news/latest-tech-innovations",
  fromCommentId = "",
  viewed = false,
  skip = 0.0
)

if response.isSome:
  let notifications = response.get()
  echo "Received notifications: ", notifications
else:
  echo "No notifications, response: ", httpResponse
[inline-code-end]

---