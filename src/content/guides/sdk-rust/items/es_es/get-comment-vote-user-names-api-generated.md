## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| dir | i32 | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_vote_names(configuration: &configuration::Configuration) -> Result<GetCommentVoteUserNamesSuccessResponse, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/2026/10/05/article-12345#comment-678".to_string(),
        dir: 1i32,
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature".to_string()),
    };
    let response: GetCommentVoteUserNamesSuccessResponse = get_comment_vote_user_names(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---