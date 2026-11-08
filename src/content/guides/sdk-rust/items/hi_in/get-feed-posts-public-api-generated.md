req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| after_id | String | नहीं |  |
| limit | i32 | नहीं |  |
| tags | Vec<String> | नहीं |  |
| sso | String | नहीं |  |
| is_crawler | bool | नहीं |  |
| include_user_info | bool | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_feed_posts_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post123".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "article".to_string()]),
        sso: Some("sso-token-xyz".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let _response = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]