## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | No |  |
| external_id | String | No |  |
| event_type | String | No |  |
| domain | String | No |  |
| attempt_count_gt | f64 | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_pending_webhook_events Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetPendingWebhookEvents200Response, Error> {
    let params: GetPendingWebhookEventsParams = GetPendingWebhookEventsParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: Some(String::from("c12345")),
        external_id: Some(String::from("article-9876")),
        event_type: Some(String::from("comment_created")),
        domain: Some(String::from("news.example.com")),
        attempt_count_gt: Some(2.0),
        skip: Some(0.0),
    };
    let response: GetPendingWebhookEvents200Response = get_pending_webhook_events(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
