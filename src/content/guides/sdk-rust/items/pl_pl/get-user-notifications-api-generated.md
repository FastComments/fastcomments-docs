## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| page_size | i32 | Nie |  |
| after_id | String | Nie |  |
| include_context | bool | Nie |  |
| after_created_at | i64 | Nie |  |
| unread_only | bool | Nie |  |
| dm_only | bool | Nie |  |
| no_dm | bool | Nie |  |
| include_translations | bool | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---