## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| badge_id | String | はい |  |
| user_id | String | いいえ |  |
| comment_id | String | いいえ |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## 例

[inline-code-attrs-start title = 'put_award_badge の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn award_badge_example() -> Result<AwardUserBadgeResponse, Error> {
    let params: PutAwardBadgeParams = PutAwardBadgeParams {
        badge_id: "community-champion".to_string(),
        user_id: Some("user-4821".to_string()),
        comment_id: Some("news/article/2026-06-18-comment-91".to_string()),
        broadcast_id: None,
        sso: Some("acme-corp-sso-token-abc123".to_string()),
    };
    let response: AwardUserBadgeResponse = put_award_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]