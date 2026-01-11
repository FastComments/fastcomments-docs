---
## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| page_size | i32 | 否 |  |
| after_id | String | 否 |  |
| include_context | bool | 否 |  |
| after_created_at | i64 | 否 |  |
| unread_only | bool | 否 |  |
| dm_only | bool | 否 |  |
| no_dm | bool | 否 |  |
| include_translations | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---