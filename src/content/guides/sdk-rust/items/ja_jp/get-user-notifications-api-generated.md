## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| page_size | i32 | いいえ |  |
| after_id | String | いいえ |  |
| include_context | bool | いいえ |  |
| after_created_at | i64 | いいえ |  |
| unread_only | bool | いいえ |  |
| dm_only | bool | いいえ |  |
| no_dm | bool | いいえ |  |
| include_translations | bool | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

---