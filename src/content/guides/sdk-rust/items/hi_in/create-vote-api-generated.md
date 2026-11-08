## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| direction | String | हाँ |  |
| user_id | String | नहीं |  |
| anon_user_id | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'create_vote उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn submit_vote() -> Result<(), Error> {
    let params = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: Some("anon-99".to_string()),
    };
    let _response = create_vote(&config, params).await?;
    Ok(())
}
[inline-code-end]