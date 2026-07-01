## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Yanıt

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_pending_webhook_event Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeletePendingWebhookEventParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "event-12345".into(),
    };
    delete_pending_webhook_event(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---