## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Ne |  |
| external_id | String | Ne |  |
| event_type | String | Ne |  |
| domain | String | Ne |  |
| attempt_count_gt | f64 | Ne |  |

## Odgovor

Vraća: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_200_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_pending_webhook_event_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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