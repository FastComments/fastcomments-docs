启用或禁用页面的通知。当用户订阅页面时，会为新的根评论创建通知，并且

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| url | String | 是 |  |
| page_title | String | 是 |  |
| subscribed_or_unsubscribed | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)