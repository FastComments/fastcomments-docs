## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| url | string | Não |  |
| pageTitle | string | Não |  |
| subscribedOrUnsubscribed | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-2025-11-22",
  url = "https://example.com/news/article-2025-11-22",
  pageTitle = "Breaking News: Market Update",
  subscribedOrUnsubscribed = "subscribed",
  sso = "sso-token-abc123"
)

if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---