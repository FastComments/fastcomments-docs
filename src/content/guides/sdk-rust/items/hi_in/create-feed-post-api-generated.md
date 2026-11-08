## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_feed_post_params | models::CreateFeedPostParams | हाँ |  |
| broadcast_id | String | नहीं |  |
| is_live | bool | नहीं |  |
| do_spam_check | bool | नहीं |  |
| skip_dup_check | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'create_feed_post उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".into(),
        create_feed_post_params: models::CreateFeedPostParams {
            text: "Launching new features".into(),
            media: vec![],
        },
        broadcast_id: Some("broadcast-2023-09".into()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let _response = create_feed_post(configuration, params).await?;
    Ok(())
}
[inline-code-end]