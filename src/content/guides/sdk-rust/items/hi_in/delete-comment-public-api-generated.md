## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| broadcast_id | String | हाँ |  |
| edit_key | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicApiDeleteCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_delete_comment_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_comment_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params = DeleteCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        broadcast_id: "news/article-6789".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response = delete_comment_public(&configuration, params).await?;
    let _deleted: PublicApiDeleteCommentResponse = response;
    Ok(())
}
[inline-code-end]