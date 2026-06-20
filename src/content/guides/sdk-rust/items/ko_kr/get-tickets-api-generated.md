---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 아니요 |  |
| state | f64 | 아니요 |  |
| skip | f64 | 아니요 |  |
| limit | f64 | 아니요 |  |

## 응답

반환: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## 예제

[inline-code-attrs-start title = 'get_tickets 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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