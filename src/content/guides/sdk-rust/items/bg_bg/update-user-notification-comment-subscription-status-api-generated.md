Активиране или деактивиране на известия за конкретен коментар.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| notification_id | String | Да |  |
| opted_in_or_out | String | Да |  |
| comment_id | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)