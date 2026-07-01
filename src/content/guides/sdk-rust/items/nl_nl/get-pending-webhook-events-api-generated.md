## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenant_id | String | Ja |  |
| comment_id | String | Nee |  |
| external_id | String | Nee |  |
| event_type | String | Nee |  |
| domain | String | Nee |  |
| attempt_count_gt | f64 | Nee |  |
| skip | f64 | Nee |  |

## Respons

Retourneert: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_pending_webhook_events'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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