---
req
tenantId
urlId
userIdWS

## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| user_id_ws | String | 是 |  |
| start_time | i64 | 是 |  |
| end_time | i64 | 否 |  |

## 回應

回傳：[`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## 範例

[inline-code-attrs-start title = 'get_event_log 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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