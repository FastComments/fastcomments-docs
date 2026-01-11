## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page_size | i32 | Nein |  |
| after_id | String | Nein |  |
| include_context | bool | Nein |  |
| after_created_at | i64 | Nein |  |
| unread_only | bool | Nein |  |
| dm_only | bool | Nein |  |
| no_dm | bool | Nein |  |
| include_translations | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---