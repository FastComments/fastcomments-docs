## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| badge_id | String | はい |  |
| user_id | String | いいえ |  |
| comment_id | String | いいえ |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## 例

[inline-code-attrs-start title = 'put_remove_badge の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PutRemoveBadgeParams = PutRemoveBadgeParams {
    badge_id: "trusted-moderator-1".to_string(),
    user_id: Some("user-82f9".to_string()),
    comment_id: Some("comment-000123".to_string()),
    broadcast_id: Some("live-broadcast-nyc-2026".to_string()),
    sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
};

let response: RemoveUserBadgeResponse = put_remove_badge(&configuration, params).await?;
[inline-code-end]

---