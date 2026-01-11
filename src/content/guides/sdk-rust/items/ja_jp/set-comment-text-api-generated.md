## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| broadcast_id | String | はい |  |
| comment_text_update_request | models::CommentTextUpdateRequest | はい |  |
| edit_key | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---