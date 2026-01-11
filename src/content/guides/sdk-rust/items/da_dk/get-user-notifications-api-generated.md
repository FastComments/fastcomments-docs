## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page_size | i32 | Nej |  |
| after_id | String | Nej |  |
| include_context | bool | Nej |  |
| after_created_at | i64 | Nej |  |
| unread_only | bool | Nej |  |
| dm_only | bool | Nej |  |
| no_dm | bool | Nej |  |
| include_translations | bool | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---