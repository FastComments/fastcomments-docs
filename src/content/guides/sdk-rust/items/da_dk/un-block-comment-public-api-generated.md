## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Ja |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Eksempel

[inline-code-attrs-start title = 'un_block_comment_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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