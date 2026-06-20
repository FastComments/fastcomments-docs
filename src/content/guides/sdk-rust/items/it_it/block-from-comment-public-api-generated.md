## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Sì |  |
| sso | String | No |  |

## Risposta

Restituisce: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di block_from_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_block_comment() -> Result<(), Error> {
    let params: BlockFromCommentPublicParams = BlockFromCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-news-20250615-9876".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams {
            reason: "Repeated harassment and targeted insults".to_string(),
            duration_hours: Some(24),
        },
        sso: Some("sso:eyJhbGciOiJIUzI1Ni...".to_string()),
    };
    let block_result: BlockSuccess = block_from_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---