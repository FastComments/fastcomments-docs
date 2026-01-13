啟用或停用特定評論的通知。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| notification_id | String | 是 |  |
| opted_in_or_out | String | 是 |  |
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)