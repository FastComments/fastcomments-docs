## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Не |  |
| external_id | String | Не |  |
| event_type | String | Не |  |
| domain | String | Не |  |
| attempt_count_gt | f64 | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pending_webhook_event_count_response.rs)

## Пример

[inline-code-attrs-start title = 'get_pending_webhook_event_count пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetPendingWebhookEventCountParams = GetPendingWebhookEventCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: Some("cmt_92a7b3".to_string()),
        external_id: Some("article-2026-06-19".to_string()),
        event_type: Some("comment.created".to_string()),
        domain: Some("acme.com".to_string()),
        attempt_count_gt: Some(1.0),
    };
    let count_response: GetPendingWebhookEventCountResponse = get_pending_webhook_event_count(cfg, params).await?;
    Ok(())
}
[inline-code-end]

---