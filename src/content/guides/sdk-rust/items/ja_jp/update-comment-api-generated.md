## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | はい |  |
| context_user_id | String | いいえ |  |
| do_spam_check | bool | いいえ |  |
| is_live | bool | いいえ |  |

## レスポンス

返却: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---