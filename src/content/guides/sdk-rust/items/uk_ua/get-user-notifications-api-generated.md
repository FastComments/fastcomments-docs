## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| page_size | i32 | Ні |  |
| after_id | String | Ні |  |
| include_context | bool | Ні |  |
| after_created_at | i64 | Ні |  |
| unread_only | bool | Ні |  |
| dm_only | bool | Ні |  |
| no_dm | bool | Ні |  |
| include_translations | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---