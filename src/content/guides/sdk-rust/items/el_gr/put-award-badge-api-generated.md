## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badge_id | String | Ναι |  |
| user_id | String | Όχι |  |
| comment_id | String | Όχι |  |
| broadcast_id | String | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα put_award_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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