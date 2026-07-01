## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| badges_user_id | String | Non |  |
| comment_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_manual_badges_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badges(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetManualBadgesForUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        badges_user_id: Some("user-42".to_string()),
        comment_id: Some("comment-987".to_string()),
        sso: Some("sso-abc123".to_string()),
    };
    let _response: GetUserManualBadgesResponse = get_manual_badges_for_user(config, params).await?;
    Ok(())
}
[inline-code-end]