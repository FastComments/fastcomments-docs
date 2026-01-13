## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| send_email | String | Non |  |

## Réponse

Retourne: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteModeratorParams = DeleteModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-1234".to_string(),
        send_email: Some("true".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_moderator(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---