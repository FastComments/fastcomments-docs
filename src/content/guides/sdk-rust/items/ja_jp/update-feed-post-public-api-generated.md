---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_id | String | はい |  |
| update_feed_post_params | models::UpdateFeedPostParams | はい |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

---