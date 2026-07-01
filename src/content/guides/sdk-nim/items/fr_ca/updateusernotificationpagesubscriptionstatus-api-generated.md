Activer ou désactiver les notifications pour une page. Lorsqu’un utilisateur s’abonne à une page, des notifications sont créées  
pour les nouveaux commentaires racine, et également

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| url | string | Non |  |
| pageTitle | string | Non |  |
| subscribedOrUnsubscribed | string | Non |  |
| sso | string = "" | Non |  |

## Réponse

Retourne : [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # traitement supplémentaire avec resp
[inline-code-end]