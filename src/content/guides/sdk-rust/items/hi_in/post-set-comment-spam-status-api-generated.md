## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| spam | bool | नहीं |  |
| perm_not_spam | bool | नहीं |  |
| broadcast_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'post_set_comment_spam_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostSetCommentSpamStatusParams {
        tenant_id: "acme-corp-tenant".into(),
        comment_id: "comment-12345".into(),
        spam: Some(true),
        perm_not_spam: Some(false),
        broadcast_id: Some("broadcast-678".into()),
        sso: Some("user@example.com".into()),
    };
    post_set_comment_spam_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]