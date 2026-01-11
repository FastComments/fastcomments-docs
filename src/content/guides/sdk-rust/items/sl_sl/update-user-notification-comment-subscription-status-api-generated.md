---
Omogočite ali onemogočite obvestila za določen komentar.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| notification_id | String | Da |  |
| opted_in_or_out | String | Da |  |
| comment_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---