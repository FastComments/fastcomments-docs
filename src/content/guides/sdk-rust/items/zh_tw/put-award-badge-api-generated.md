## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| badge_id | String | 是 |  |
| user_id | String | 否 |  |
| comment_id | String | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## 範例

[inline-code-attrs-start title = 'put_award_badge 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---