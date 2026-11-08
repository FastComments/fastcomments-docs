## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| dir | i32 | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_comment_vote_user_names उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn demo(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-123".to_string(),
        dir: 1,
        sso: Some("user-sso-id".to_string()),
    };
    let _response: GetCommentVoteUserNamesSuccessResponse =
        get_comment_vote_user_names(config, params).await?;
    Ok(())
}
[inline-code-end]