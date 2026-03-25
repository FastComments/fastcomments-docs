## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Evet |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'block_from_comment_public Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run_block_example() -> Result<(), Error> {
    let params: BlockFromCommentPublicParams = BlockFromCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026-03-25-12345".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams {
            reason: "Repeated promotional links".to_string(),
            reporter_id: Some("reader-2048".to_string()),
            permanent: Some(false),
        },
        sso: Some("sso:user:acme:2048".to_string()),
    };
    let response: BlockFromCommentPublic200Response = block_from_comment_public(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---