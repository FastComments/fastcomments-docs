## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Não |  |
| external_id | String | Não |  |
| event_type | String | Não |  |
| domain | String | Não |  |
| attempt_count_gt | f64 | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_events_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_pending_webhook_events Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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