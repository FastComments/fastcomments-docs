req
tenantId
urlId
userIdWS

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| user_id_ws | String | はい |  |
| start_time | i64 | はい |  |
| end_time | i64 | いいえ |  |

## レスポンス

戻り値: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## 例

[inline-code-attrs-start title = 'get_global_event_log の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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