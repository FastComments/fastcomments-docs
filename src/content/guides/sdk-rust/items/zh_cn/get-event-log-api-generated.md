req
tenantId
urlId
userIdWS

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| user_id_ws | String | 是 |  |
| start_time | i64 | 是 |  |
| end_time | i64 | 是 |  |

## 响应

返回： [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_event_log 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEventLogParams = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-2023-01-01".to_string(),
        user_id_ws: "user-12345-ws".to_string(),
        start_time: 1672531200i64,
        end_time: 1672617599i64,
        include_details: Some(true),
    };
    let response: GetEventLog200Response = get_event_log(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---