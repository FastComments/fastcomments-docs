## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | Sim |  |
| user_id | String | Não |  |
| anon_user_id | String | Não |  |

## Resposta

Retorna: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de un_block_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<UnblockSuccess, Error> {
    let params = UnBlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        un_block_from_comment_params: models::UnBlockFromCommentParams::default(),
        user_id: Some("user-67890".to_string()),
        anon_user_id: Some("anon-abcde".to_string()),
    };
    let result = un_block_user_from_comment(config, params).await?;
    Ok(result)
}
[inline-code-end]