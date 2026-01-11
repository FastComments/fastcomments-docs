---
## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| page_size | i32 | Hayır |  |
| after_id | String | Hayır |  |
| include_context | bool | Hayır |  |
| after_created_at | i64 | Hayır |  |
| unread_only | bool | Hayır |  |
| dm_only | bool | Hayır |  |
| no_dm | bool | Hayır |  |
| include_translations | bool | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---