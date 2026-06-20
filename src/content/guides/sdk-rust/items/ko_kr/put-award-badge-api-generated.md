## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badge_id | String | 예 |  |
| user_id | String | 아니요 |  |
| comment_id | String | 아니요 |  |
| broadcast_id | String | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## 예제

[inline-code-attrs-start title = 'put_award_badge 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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