---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| badge_id | String | Oui |  |
| user_id | String | Non |  |
| comment_id | String | Non |  |
| broadcast_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de put_remove_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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