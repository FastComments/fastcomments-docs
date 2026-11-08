## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| direction | String | नहीं |  |
| broadcast_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'post_vote उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let cfg = configuration::Configuration::default();
    let params = PostVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        direction: Some("up".to_string()),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: None,
    };
    let _response = post_vote(&cfg, params).await?;
    Ok(())
}
[inline-code-end]