## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| state | f64 | 否 |  |
| skip | f64 | 否 |  |
| limit | f64 | 否 |  |

## 回應

回傳：[`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tickets 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("journalist-42")),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(50.0),
    };
    let tickets: GetTicketsResponse = get_tickets(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---