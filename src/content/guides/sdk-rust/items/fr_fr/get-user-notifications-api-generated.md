## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| page_size | i32 | Non |  |
| after_id | String | Non |  |
| include_context | bool | Non |  |
| after_created_at | i64 | Non |  |
| unread_only | bool | Non |  |
| dm_only | bool | Non |  |
| no_dm | bool | Non |  |
| include_translations | bool | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---