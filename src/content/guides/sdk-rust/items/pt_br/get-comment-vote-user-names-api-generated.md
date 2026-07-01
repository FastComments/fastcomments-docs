## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| dir | i32 | Yes |  |
| sso | String | No |  |

## Resposta

Retorna: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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