Abilita o disabilita le notifiche per un commento specifico.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| notification_id | String | Sì |  |
| opted_in_or_out | String | Sì |  |
| comment_id | String | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---