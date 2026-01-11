Омогућите или онемогућите обавештења за страницу. Када су корисници претплаћени на страницу, обавештења се креирају за нове root коментаре, и такође

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| url | String | Да |  |
| page_title | String | Да |  |
| subscribed_or_unsubscribed | String | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)