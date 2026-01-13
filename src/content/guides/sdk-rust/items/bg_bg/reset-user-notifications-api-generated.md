## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| after_id | String | Не |  |
| after_created_at | i64 | Не |  |
| unread_only | bool | Не |  |
| dm_only | bool | Не |  |
| no_dm | bool | Не |  |
| sso | String | Не |  |

## Отговор

Връща: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

---