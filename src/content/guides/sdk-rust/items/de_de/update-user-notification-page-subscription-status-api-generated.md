Benachrichtigungen für eine Seite aktivieren oder deaktivieren. Wenn Benutzer eine Seite abonniert haben, werden Benachrichtigungen für neue Root-Kommentare erstellt, und auch

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| url | String | Ja |  |
| page_title | String | Ja |  |
| subscribed_or_unsubscribed | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)