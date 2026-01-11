## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page_size | i32 | Nee |  |
| after_id | String | Nee |  |
| include_context | bool | Nee |  |
| after_created_at | i64 | Nee |  |
| unread_only | bool | Nee |  |
| dm_only | bool | Nee |  |
| no_dm | bool | Nee |  |
| include_translations | bool | Nee |  |
| sso | String | Nee |  |

## Antwoord

Retourneert: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---