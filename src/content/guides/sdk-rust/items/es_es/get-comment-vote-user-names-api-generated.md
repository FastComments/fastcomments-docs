---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| dir | i32 | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn demo(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-123".to_string(),
        dir: 1,
        sso: Some("user-sso-id".to_string()),
    };
    let _response: GetCommentVoteUserNamesSuccessResponse =
        get_comment_vote_user_names(config, params).await?;
    Ok(())
}
[inline-code-end]

---