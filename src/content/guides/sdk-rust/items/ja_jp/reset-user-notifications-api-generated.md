## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| after_id | String | いいえ |  |
| after_created_at | i64 | いいえ |  |
| unread_only | bool | いいえ |  |
| dm_only | bool | いいえ |  |
| no_dm | bool | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

---