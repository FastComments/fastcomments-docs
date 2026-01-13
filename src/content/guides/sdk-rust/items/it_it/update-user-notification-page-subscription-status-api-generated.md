Abilita o disabilita le notifiche per una pagina. Quando gli utenti sono iscritti a una pagina, vengono create notifiche per nuovi commenti di livello principale, e anche

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| url | String | Sì |  |
| page_title | String | Sì |  |
| subscribed_or_unsubscribed | String | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)