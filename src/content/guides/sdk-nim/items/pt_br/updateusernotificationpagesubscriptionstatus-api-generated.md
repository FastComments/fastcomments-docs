Habilite ou desabilite notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas  
para novos comentários principais, e também

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| url | string | No |  |
| pageTitle | string | No |  |
| subscribedOrUnsubscribed | string | No |  |
| sso | string = "" | No |  |

## Resposta

Retorna: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Exemplo

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Exemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-456",
  url = "https://example.com/news/article-456",
  pageTitle = "Breaking News: Something Happened",
  subscribedOrUnsubscribed = "subscribed",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  # processamento adicional com resp
[inline-code-end]