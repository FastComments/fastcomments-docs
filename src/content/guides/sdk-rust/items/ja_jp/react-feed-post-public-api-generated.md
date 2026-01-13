## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_id | String | はい |  |
| react_body_params | models::ReactBodyParams | はい |  |
| is_undo | bool | いいえ |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)

---