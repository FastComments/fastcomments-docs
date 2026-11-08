## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## उदाहरण

[inline-code-attrs-start title = 'block_from_comment_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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