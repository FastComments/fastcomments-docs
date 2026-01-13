---
Активиране или деактивиране на известия за страница. Когато потребителите са абонирани за страница, се създават известия за нови коренни коментари, както и

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| url | String | Да |  |
| page_title | String | Да |  |
| subscribed_or_unsubscribed | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---