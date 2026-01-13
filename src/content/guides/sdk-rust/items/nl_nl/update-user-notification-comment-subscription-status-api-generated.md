Schakel meldingen voor een specifieke reactie in of uit.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| notification_id | String | Ja |  |
| opted_in_or_out | String | Ja |  |
| comment_id | String | Ja |  |
| sso | String | Nee |  |

## Respons

Geeft terug: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---