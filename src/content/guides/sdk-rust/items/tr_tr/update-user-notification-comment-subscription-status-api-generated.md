---
Belirli bir yorum için bildirimleri etkinleştirin veya devre dışı bırakın.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| notification_id | String | Evet |  |
| opted_in_or_out | String | Evet |  |
| comment_id | String | Evet |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---