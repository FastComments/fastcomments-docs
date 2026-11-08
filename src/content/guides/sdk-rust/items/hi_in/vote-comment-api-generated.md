## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| url_id | String | हाँ |  |
| broadcast_id | String | हाँ |  |
| vote_body_params | models::VoteBodyParams | हाँ |  |
| session_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'vote_comment उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();

    let vote_body = models::VoteBodyParams {
        vote_type: "upvote".to_string(),
        weight: 1,
    };

    let params = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-67890".to_string(),
        vote_body_params: vote_body,
        session_id: Some("session-abcde".to_string()),
        sso: None,
    };

    let _response = vote_comment(&config, params).await?;
    Ok(())
}
[inline-code-end]