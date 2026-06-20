req
tenantId
urlId
userIdWS

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| user_id_ws | String | Sí |  |
| start_time | i64 | Sí |  |
| end_time | i64 | No |  |

## Respuesta

Devuelve: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_global_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_events() -> Result<GetEventLogResponse, Error> {
    let params: GetGlobalEventLogParams = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        user_id_ws: "user-42-ws".to_string(),
        start_time: 1688208000i64,
        end_time: Some(1688294400i64),
    };
    let response: GetEventLogResponse = get_global_event_log(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---