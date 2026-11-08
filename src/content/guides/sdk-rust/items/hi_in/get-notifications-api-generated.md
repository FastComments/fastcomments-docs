## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| user_id | String | नहीं |  |
| url_id | String | नहीं |  |
| from_comment_id | String | नहीं |  |
| viewed | bool | नहीं |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_notifications उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-123".to_string()),
        url_id: Some("news/article".to_string()),
        from_comment_id: Some("cmt-456".to_string()),
        viewed: Some(true),
        skip: Some(0.0),
    };
    let _response = get_notifications(configuration, params).await?;
    Ok(())
}
[inline-code-end]