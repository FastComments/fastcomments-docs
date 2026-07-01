## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| comment_id | String | Não |  |
| external_id | String | Não |  |
| event_type | String | Não |  |
| domain | String | Não |  |
| attempt_count_gt | f64 | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pending_webhook_event_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("comment-12345".to_string()),
        external_id: Some("ext-98765".to_string()),
        event_type: Some("comment_created".to_string()),
        domain: Some("news.example.com".to_string()),
        attempt_count_gt: Some(2.0),
    };
    let _response = get_pending_webhook_event_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---