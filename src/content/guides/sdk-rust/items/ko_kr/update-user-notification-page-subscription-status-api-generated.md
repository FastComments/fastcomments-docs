---
페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새 루트 댓글에 대해 알림이 생성되며, 또한

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| url | String | Yes |  |
| page_title | String | Yes |  |
| subscribed_or_unsubscribed | String | Yes |  |
| sso | String | No |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---