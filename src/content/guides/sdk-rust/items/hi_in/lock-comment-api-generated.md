## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| broadcast_id | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'lock_comment उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9876".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _resp = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]