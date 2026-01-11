Включает или отключает уведомления для страницы. Когда пользователи подписаны на страницу, уведомления создаются
для новых корневых комментариев, а также

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| url | String | Да |  |
| page_title | String | Да |  |
| subscribed_or_unsubscribed | String | Да |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---