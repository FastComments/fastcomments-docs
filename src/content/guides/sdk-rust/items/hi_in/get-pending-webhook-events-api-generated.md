## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | नहीं |  |
| external_id | String | नहीं |  |
| event_type | String | नहीं |  |
| domain | String | नहीं |  |
| attempt_count_gt | f64 | नहीं |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_pending_webhook_events उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn demo() -> Result<(), Error> {
    let params = GetPendingWebhookEventsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("comment-123".to_string()),
        external_id: Some("external-789".to_string()),
        event_type: Some("comment_created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(1.0),
        skip: Some(0.0),
    };
    let _response = get_pending_webhook_events(&config, params).await?;
    Ok(())
}
[inline-code-end]