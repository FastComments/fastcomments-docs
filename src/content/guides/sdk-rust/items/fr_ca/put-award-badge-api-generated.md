## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| badge_id | String | Oui |  |
| user_id | String | Non |  |
| comment_id | String | Non |  |
| broadcast_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple put_award_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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