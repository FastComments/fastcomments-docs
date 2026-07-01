## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| badge_id | String | Non |  |
| displayed_on_comments | bool | Non |  |
| limit | f64 | Non |  |
| skip | f64 | Non |  |

## Réponse

Retourne : [`ApiGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badges_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_user_badges'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badges(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBadgesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
        badge_id: Some("top-commenter".to_string()),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let _response = get_user_badges(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---