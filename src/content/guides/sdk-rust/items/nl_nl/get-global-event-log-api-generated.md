req
tenantId
urlId
userIdWS

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| user_id_ws | String | Ja |  |
| start_time | i64 | Ja |  |
| end_time | i64 | Nee |  |

## Antwoord

Geeft terug: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_global_event_log Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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