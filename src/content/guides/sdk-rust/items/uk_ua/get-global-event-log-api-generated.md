req
tenantId
urlId
userIdWS

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| user_id_ws | String | Так |  |
| start_time | i64 | Так |  |
| end_time | i64 | Ні |  |

## Відповідь

Повертає: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_global_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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