## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 아니오 |  |
| state | f64 | 아니오 |  |
| skip | f64 | 아니오 |  |
| limit | f64 | 아니오 |  |

## 응답

반환: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## 예시

[inline-code-attrs-start title = 'get_tickets 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tickets(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTicketsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(20.0),
    };
    let _response: GetTicketsResponse = get_tickets(configuration, params).await?;
    Ok(())
}
[inline-code-end]