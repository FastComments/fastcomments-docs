## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| badge_id | String | Ja |  |
| user_id | String | Nee |  |
| comment_id | String | Nee |  |
| broadcast_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'put_remove_badge Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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