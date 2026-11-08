## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ApiGetCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comment_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_comment उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<(), Error> {
    let params = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        include_deleted: Some(false),
    };

    let _response: ApiGetCommentResponse = get_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]