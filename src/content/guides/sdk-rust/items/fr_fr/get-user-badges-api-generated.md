---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| badge_id | String | Non |  |
| displayed_on_comments | bool | Non |  |
| limit | f64 | Non |  |
| skip | f64 | Non |  |

## Réponse

Renvoie : [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badges_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_badges'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserBadgesParams = GetUserBadgesParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-9876")),
        badge_id: Some(String::from("top-reviewer")),
        displayed_on_comments: Some(true),
        limit: Some(50.0),
        skip: Some(0.0),
    };
    let response: GetUserBadges200Response = get_user_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---