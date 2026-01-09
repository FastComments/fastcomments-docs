Activer ou désactiver les notifications pour une page. Lorsque des utilisateurs sont abonnés à une page, des notifications sont créées pour les nouveaux commentaires racines, et aussi

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| url | string | Oui |  |
| pageTitle | string | Oui |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)