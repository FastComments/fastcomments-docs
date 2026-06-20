---
## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badge_id | String | Evet |  |
| user_id | String | Hayır |  |
| comment_id | String | Hayır |  |
| broadcast_id | String | Hayır |  |
| sso | String | Hayır |  |

## Response

Döndürür: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## Example

[inline-code-attrs-start title = 'put_award_badge Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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