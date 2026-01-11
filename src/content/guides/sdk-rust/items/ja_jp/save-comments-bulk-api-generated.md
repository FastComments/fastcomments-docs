---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_comment_params | Vec<models::CreateCommentParams> | はい |  |
| is_live | bool | いいえ |  |
| do_spam_check | bool | いいえ |  |
| send_emails | bool | いいえ |  |
| populate_notifications | bool | いいえ |  |

## レスポンス

戻り値: `Vec<models::SaveComment200Response>`

---