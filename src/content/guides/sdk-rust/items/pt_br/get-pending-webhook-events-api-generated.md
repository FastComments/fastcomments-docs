## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| comment_id | String | Não |  |
| external_id | String | Não |  |
| event_type | String | Não |  |
| domain | String | Não |  |
| attempt_count_gt | f64 | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_pending_webhook_events'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_pending_webhook_events() -> Result<GetPendingWebhookEvents200Response, Error> {
    let params: GetPendingWebhookEventsParams = GetPendingWebhookEventsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("cmt-2026-04-01-001".to_string()),
        external_id: Some("news/article-42".to_string()),
        event_type: Some("comment.created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(1.0),
        skip: Some(0.0),
    };
    let response: GetPendingWebhookEvents200Response = get_pending_webhook_events(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---