为特定评论启用或禁用通知。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| notification_id | String | 是 |  |
| opted_in_or_out | String | 是 |  |
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回：[`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---