req
tenantId
afterId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| after_id | String | いいえ |  |
| limit | i32 | いいえ |  |
| tags | Vec<String> | いいえ |  |
| sso | String | いいえ |  |
| is_crawler | bool | いいえ |  |
| include_user_info | bool | いいえ |  |

## レスポンス

返却: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---