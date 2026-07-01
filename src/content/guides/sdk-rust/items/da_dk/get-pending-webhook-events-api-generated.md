## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Nej |  |
| external_id | String | Nej |  |
| event_type | String | Nej |  |
| domain | String | Nej |  |
| attempt_count_gt | f64 | Nej |  |
| skip | f64 | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_pending_webhook_events Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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