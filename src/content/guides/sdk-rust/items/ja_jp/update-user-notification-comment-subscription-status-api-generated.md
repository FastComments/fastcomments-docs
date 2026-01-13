特定のコメントに対する通知を有効または無効にします。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| notification_id | String | はい |  |
| opted_in_or_out | String | はい |  |
| comment_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)