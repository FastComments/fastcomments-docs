## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |

## 応答

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'delete_pending_webhook_event の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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