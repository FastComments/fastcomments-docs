## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| post_id | String | हाँ |  |
| update_feed_post_params | models::UpdateFeedPostParams | हाँ |  |
| broadcast_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_feed_post_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".into(),
    post_id: "news/article-123".into(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Updated Headline".into()),
        content: Some("Revised content of the article with latest information.".into()),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-001".into()),
    sso: Some("sso-token-abc123".into()),
};

let response: CreateFeedPostResponse = update_feed_post_public(&configuration, params).await?;
[inline-code-end]