ページの通知を有効または無効にします。ユーザーがページを購読している場合、通知が作成され
新しいルートコメントに対して、また

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| url | String | はい |  |
| page_title | String | はい |  |
| subscribed_or_unsubscribed | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

返却値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---