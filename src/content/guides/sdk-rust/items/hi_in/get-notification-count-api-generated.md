## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| user_id | String | नहीं |  |
| url_id | String | नहीं |  |
| from_comment_id | String | नहीं |  |
| viewed | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_notification_count उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("john.doe".to_string()),
        url_id: Some("blog/post-123".to_string()),
        from_comment_id: Some("comment789".to_string()),
        viewed: Some(true),
    };
    let _response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]