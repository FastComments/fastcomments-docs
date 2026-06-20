req
tenantId
urlId
userIdWS

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| user_id_ws | String | 是 |  |
| start_time | i64 | 是 |  |
| end_time | i64 | 否 |  |

## 回應

回傳: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## 範例

[inline-code-attrs-start title = 'get_global_event_log 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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