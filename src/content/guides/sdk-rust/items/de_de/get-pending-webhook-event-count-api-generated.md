## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Nein |  |
| external_id | String | Nein |  |
| event_type | String | Nein |  |
| domain | String | Nein |  |
| attempt_count_gt | f64 | Nein |  |

## Antwort

Gibt zurück: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_pending_webhook_event_count Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetPendingWebhookEventCount200Response, Error> {
    let params: GetPendingWebhookEventCountParams = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("cmt-12345".to_string()),
        external_id: Some("news/article-98765".to_string()),
        event_type: Some("comment.created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(2.0),
    };
    let response: GetPendingWebhookEventCount200Response =
        get_pending_webhook_event_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---