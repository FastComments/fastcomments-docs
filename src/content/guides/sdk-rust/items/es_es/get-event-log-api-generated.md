req
tenantId
urlId
userIdWS

## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| user_id_ws | String | Sí |  |
| start_time | i64 | Sí |  |
| end_time | i64 | No |  |

## Respuesta

Devuelve: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_event_log() -> Result<(), Error> {
    let params = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2024-product-launch".to_string(),
        user_id_ws: "user_98765_ws".to_string(),
        start_time: 1710700800i64,
        end_time: Some(1710787200i64),
    };
    let response: GetEventLogResponse = get_event_log(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---