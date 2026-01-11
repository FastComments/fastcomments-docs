## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| after_id | String | 否 |  |
| after_created_at | i64 | 否 |  |
| unread_only | bool | 否 |  |
| dm_only | bool | 否 |  |
| no_dm | bool | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

---