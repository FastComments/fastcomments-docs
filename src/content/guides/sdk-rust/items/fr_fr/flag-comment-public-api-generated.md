## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| is_flagged | bool | Oui |  |
| sso | String | Non |  |

## Réponse

Retourne : [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemple

[inline-code-attrs-start title = 'flag_comment_public Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = FlagCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        is_flagged: true,
        sso: Some("user-sso-token".to_string()),
    };
    flag_comment_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---