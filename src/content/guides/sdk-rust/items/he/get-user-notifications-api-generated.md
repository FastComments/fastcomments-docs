## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| page_size | i32 | לא |  |
| after_id | String | לא |  |
| include_context | bool | לא |  |
| after_created_at | i64 | לא |  |
| unread_only | bool | לא |  |
| dm_only | bool | לא |  |
| no_dm | bool | לא |  |
| include_translations | bool | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---