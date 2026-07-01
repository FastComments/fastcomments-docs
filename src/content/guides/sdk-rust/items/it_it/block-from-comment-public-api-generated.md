## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio block_from_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = BlockFromCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-98765".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams::default(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _result: BlockSuccess = block_from_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]