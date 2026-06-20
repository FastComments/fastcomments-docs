## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Ja |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Voorbeeld

[inline-code-attrs-start title = 'block_from_comment_public Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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