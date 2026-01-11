---
Activer ou désactiver les notifications pour un commentaire spécifique.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| notification_id | String | Oui |  |
| opted_in_or_out | String | Oui |  |
| comment_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---