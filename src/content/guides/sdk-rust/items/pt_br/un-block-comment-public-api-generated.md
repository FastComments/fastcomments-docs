## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Sim |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Exemplo

[inline-code-attrs-start title = 'un_block_comment_public Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UnBlockCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams::default(),
        sso: Some("user-sso-token".to_string()),
    };
    let _result: UnblockSuccess = un_block_comment_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]