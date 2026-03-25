## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 아니요 |  |
| state | f64 | 아니요 |  |
| skip | f64 | 아니요 |  |
| limit | f64 | 아니요 |  |

## 응답

반환: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_tickets 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        user_id: Some("user-9876".to_owned()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(25.0),
    };
    let tickets: GetTickets200Response = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---