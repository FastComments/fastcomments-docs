## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nej |  |
| after_created_at | i64 | Nej |  |
| unread_only | bool | Nej |  |
| dm_only | bool | Nej |  |
| no_dm | bool | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)