## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Retourne : [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let response = get_logs(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---