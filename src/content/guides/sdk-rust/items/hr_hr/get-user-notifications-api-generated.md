## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| page_size | i32 | Ne |  |
| after_id | String | Ne |  |
| include_context | bool | Ne |  |
| after_created_at | i64 | Ne |  |
| unread_only | bool | Ne |  |
| dm_only | bool | Ne |  |
| no_dm | bool | Ne |  |
| include_translations | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---