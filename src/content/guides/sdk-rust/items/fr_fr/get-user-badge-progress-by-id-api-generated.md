## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Retourne : [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_user_badge_progress_by_id'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserBadgeProgressByIdParams {
        tenant_id: "acme-corp".to_string(),
        id: "user-12345".to_string(),
    };
    let _response = get_user_badge_progress_by_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]