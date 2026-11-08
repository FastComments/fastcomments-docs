## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| edit_key | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicApiGetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_get_comment_text_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_comment_text उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        edit_key: Some("edit-key-abc".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = get_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]