## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| page_size | i32 | Нет |  |
| after_id | String | Нет |  |
| include_context | bool | Нет |  |
| after_created_at | i64 | Нет |  |
| unread_only | bool | Нет |  |
| dm_only | bool | Нет |  |
| no_dm | bool | Нет |  |
| include_translations | bool | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---