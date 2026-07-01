req
tenantId
urlId
userIdWS

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| user_id_ws | String | 是 |  |
| start_time | i64 | 是 |  |
| end_time | i64 | 否 |  |

## 响应

返回：[`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## 示例

[inline-code-attrs-start title = 'get_global_event_log 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        user_id_ws: "user-12345".to_string(),
        start_time: 1_680_000_000,
        end_time: Some(1_680_864_000),
    };
    let _response = get_global_event_log(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---