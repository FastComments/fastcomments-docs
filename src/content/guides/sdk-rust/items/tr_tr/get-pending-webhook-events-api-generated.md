## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Hayır |  |
| external_id | String | Hayır |  |
| event_type | String | Hayır |  |
| domain | String | Hayır |  |
| attempt_count_gt | f64 | Hayır |  |
| skip | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_pending_webhook_events Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPendingWebhookEventsParams = GetPendingWebhookEventsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: Some("cmt-12345".to_string()),
    external_id: Some("article-98765".to_string()),
    event_type: Some("comment.create".to_string()),
    domain: Some("news.example.com".to_string()),
    attempt_count_gt: Some(1.0),
    skip: Some(0.0),
};

let pending: GetPendingWebhookEvents200Response = get_pending_webhook_events(&configuration, params).await?;
[inline-code-end]