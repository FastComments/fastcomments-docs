## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Primer

[inline-code-attrs-start title = 'Primer un_block_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---