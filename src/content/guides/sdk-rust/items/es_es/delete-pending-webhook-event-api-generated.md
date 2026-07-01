## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |

## Respuesta

Devuelve: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_pending_webhook_event'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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